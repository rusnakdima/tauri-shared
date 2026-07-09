use crate::sync::MongoBridge;
use crate::AppError;
use crate::Result;
use nosql_orm::prelude::DatabaseProvider;
use nosql_orm::prelude::SchemaIntrospection;
use nosql_orm::providers::json::JsonProvider;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

type LastSyncState = HashMap<String, HashMap<String, Value>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SyncOperation {
  Insert {
    collection: String,
    id: String,
    doc: Value,
    timestamp: i64,
  },
  Update {
    collection: String,
    id: String,
    doc: Value,
    timestamp: i64,
  },
  Delete {
    collection: String,
    id: String,
    timestamp: i64,
  },
}

impl SyncOperation {
  pub fn timestamp(&self) -> i64 {
    match self {
      SyncOperation::Insert { timestamp, .. } => *timestamp,
      SyncOperation::Update { timestamp, .. } => *timestamp,
      SyncOperation::Delete { timestamp, .. } => *timestamp,
    }
  }
}

pub struct SyncQueue {
  operations: RwLock<Vec<SyncOperation>>,
}

impl SyncQueue {
  pub fn new() -> Self {
    Self {
      operations: RwLock::new(Vec::new()),
    }
  }

  pub fn push(&self, op: SyncOperation) {
    let mut ops = self.operations.write().unwrap();
    ops.push(op);
  }

  pub fn pop(&self) -> Option<SyncOperation> {
    let mut ops = self.operations.write().unwrap();
    if ops.is_empty() {
      None
    } else {
      Some(ops.remove(0))
    }
  }

  pub fn get_all(&self) -> Vec<SyncOperation> {
    let ops = self.operations.read().unwrap();
    ops.clone()
  }

  pub fn len(&self) -> usize {
    let ops = self.operations.read().unwrap();
    ops.len()
  }

  pub fn clear(&self) {
    let mut ops = self.operations.write().unwrap();
    ops.clear();
  }
}

impl Default for SyncQueue {
  fn default() -> Self {
    Self::new()
  }
}

pub struct SyncEngine {
  queue: SyncQueue,
  local_db: JsonProvider,
  mongo_bridge: Option<MongoBridge>,
  last_sync_state: RwLock<LastSyncState>,
}

impl SyncEngine {
  pub fn new(local_db: JsonProvider) -> Self {
    Self {
      queue: SyncQueue::new(),
      local_db,
      mongo_bridge: None,
      last_sync_state: RwLock::new(HashMap::new()),
    }
  }

  pub fn set_mongo_bridge(&mut self, bridge: MongoBridge) {
    self.mongo_bridge = Some(bridge);
  }

  pub fn queue_operation(&self, op: SyncOperation) {
    self.queue.push(op);
  }

  pub async fn sync_to_cloud(&self) -> Result<()> {
    let Some(ref bridge) = self.mongo_bridge else {
      return Err(AppError::Internal(
        "MongoDB bridge not configured".to_string(),
      ));
    };

    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_millis() as i64;

    let old_sync_state = self.last_sync_state.read().unwrap().clone();
    let mut new_sync_state: LastSyncState = HashMap::new();

    let collections_meta = self
      .local_db
      .list_collections()
      .await
      .map_err(AppError::from)?;
    for coll_meta in collections_meta {
      let collection = &coll_meta.name;
      let docs = self
        .local_db
        .find_all(collection)
        .await
        .map_err(AppError::from)?;
      let mut coll_sync = HashMap::new();

      for doc in docs {
        let id = doc
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or("")
          .to_string();
        let id_clone = id.clone();
        let doc_clone = doc.clone();

        let old_doc = old_sync_state
          .get(collection)
          .and_then(|c| c.get(&id_clone));

        if old_doc.map_or(true, |old| old != &doc_clone) {
          if old_doc.is_some() {
            self.queue.push(SyncOperation::Update {
              collection: collection.clone(),
              id: id.clone(),
              doc: doc_clone.clone(),
              timestamp: now,
            });
          } else {
            self.queue.push(SyncOperation::Insert {
              collection: collection.clone(),
              id: id.clone(),
              doc: doc_clone.clone(),
              timestamp: now,
            });
          }
        }

        coll_sync.insert(id_clone, doc_clone);
      }

      new_sync_state.insert(collection.clone(), coll_sync);
    }

    for (collection, old_docs) in &old_sync_state {
      if !new_sync_state.contains_key(collection) {
        for (id, _) in old_docs {
          self.queue.push(SyncOperation::Delete {
            collection: collection.clone(),
            id: id.clone(),
            timestamp: now,
          });
        }
      } else {
        for (id, _) in old_docs {
          if !new_sync_state
            .get(collection)
            .map(|c| c.contains_key(id))
            .unwrap_or(false)
          {
            self.queue.push(SyncOperation::Delete {
              collection: collection.clone(),
              id: id.clone(),
              timestamp: now,
            });
          }
        }
      }
    }

    while let Some(op) = self.queue.pop() {
      match op {
        SyncOperation::Insert {
          collection,
          id,
          doc,
          ..
        } => {
          bridge
            .insert(&collection, &id, doc)
            .await
            .map_err(|e| AppError::Internal(e))?;
        }
        SyncOperation::Update {
          collection,
          id,
          doc,
          ..
        } => {
          bridge
            .update(&collection, &id, doc)
            .await
            .map_err(|e| AppError::Internal(e))?;
        }
        SyncOperation::Delete { collection, id, .. } => {
          bridge
            .delete(&collection, &id)
            .await
            .map_err(|e| AppError::Internal(e))?;
        }
      }
    }

    *self.last_sync_state.write().unwrap() = new_sync_state;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sync_queue_add() {
    let queue = SyncQueue::new();
    queue.push(SyncOperation::Insert {
      collection: "test".to_string(),
      id: "1".to_string(),
      doc: serde_json::json!({"name": "test"}),
      timestamp: 1234567890,
    });
    assert_eq!(queue.len(), 1);
  }

  #[test]
  fn test_sync_queue_order() {
    let queue = SyncQueue::new();
    queue.push(SyncOperation::Insert {
      collection: "test".to_string(),
      id: "1".to_string(),
      doc: serde_json::json!({}),
      timestamp: 1000,
    });
    queue.push(SyncOperation::Insert {
      collection: "test".to_string(),
      id: "2".to_string(),
      doc: serde_json::json!({}),
      timestamp: 2000,
    });
    let ops = queue.get_all();
    assert!(ops[0].timestamp() < ops[1].timestamp());
  }
}
