use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub signal_metrics: String,
    pub metadata: HashMap<String, String>,
}

impl NodeInfo {
    pub fn new(id: i32, name: String, position: String, signal_metrics: String) -> Self {
        Self {
            id,
            name,
            position,
            signal_metrics,
            metadata: HashMap::new(),
        }
    }
}
