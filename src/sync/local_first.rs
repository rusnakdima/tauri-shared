use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::RwLock;

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
}

impl SyncEngine {
    pub fn new(local_db: crate::storage::JsonDb) -> Self {
        Self {
            queue: SyncQueue::new(),
            local_db,
            mongo_bridge: None,
        }
    }

    pub fn set_mongo_bridge(&mut self, bridge: crate::sync::MongoBridge) {
        self.mongo_bridge = Some(bridge);
    }

    pub fn queue_operation(&self, op: SyncOperation) {
        self.queue.push(op);
    }

    pub fn sync_to_cloud(&self) -> Result<(), String> {
        let Some(ref bridge) = self.mongo_bridge else {
            return Err("MongoDB bridge not configured".to_string());
        };

        while let Some(op) = self.queue.pop() {
            match op {
                SyncOperation::Insert { collection, id, doc, .. } => {
                    bridge.insert(&collection, &id, doc)?;
                }
                SyncOperation::Update { collection, id, doc, .. } => {
                    bridge.update(&collection, &id, doc)?;
                }
                SyncOperation::Delete { collection, id, .. } => {
                    bridge.delete(&collection, &id)?;
                }
            }
        }
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