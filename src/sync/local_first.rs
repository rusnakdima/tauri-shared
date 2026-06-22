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
    local_db: crate::storage::JsonDb,
    mongo_bridge: Option<crate::sync::MongoBridge>,
    last_sync_state: RwLock<LastSyncState>,
}

impl SyncEngine {
    pub fn new(local_db: crate::storage::JsonDb) -> Self {
        Self {
            queue: SyncQueue::new(),
            local_db,
            mongo_bridge: None,
            last_sync_state: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_mongo_bridge(&mut self, bridge: crate::sync::MongoBridge) {
        self.mongo_bridge = Some(bridge);
    }

    pub fn queue_operation(&self, op: SyncOperation) {
        self.queue.push(op);
    }

    pub async fn sync_to_cloud(&self) -> Result<(), String> {
        let Some(ref bridge) = self.mongo_bridge else {
            return Err("MongoDB bridge not configured".to_string());
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let old_sync_state = self.last_sync_state.read().unwrap().clone();
        let mut new_sync_state: LastSyncState = HashMap::new();

        let collections = self.local_db.collections();
        for collection in collections {
            let docs = self.local_db.find_all_with_id(&collection);
            let mut coll_sync = HashMap::new();

            for (id, doc) in docs {
                let id_clone = id.clone();
                let doc_clone = doc.clone();

                let old_doc = old_sync_state
                    .get(&collection)
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

            new_sync_state.insert(collection, coll_sync);
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
                    bridge.insert(&collection, &id, doc).await?;
                }
                SyncOperation::Update {
                    collection,
                    id,
                    doc,
                    ..
                } => {
                    bridge.update(&collection, &id, doc).await?;
                }
                SyncOperation::Delete { collection, id, .. } => {
                    bridge.delete(&collection, &id).await?;
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
