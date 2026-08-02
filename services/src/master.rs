use distr_core::{NodeInfo, RaftStatus, SystemMetrics};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct MasterNode {
    pub id: i32,
    pub name: String,
    pub nodes: Arc<Mutex<HashMap<i32, NodeInfo>>>,
    pub raft_statuses: Arc<Mutex<Vec<RaftStatus>>>,
    pub system_metrics: Arc<Mutex<HashMap<i32, SystemMetrics>>>,
}

impl MasterNode {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            nodes: Arc::new(Mutex::new(HashMap::new())),
            raft_statuses: Arc::new(Mutex::new(Vec::new())),
            system_metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node: NodeInfo) {
        let mut nodes = self.nodes.lock().await;
        nodes.insert(node.id, node);
    }

    pub async fn update_system_metrics(&self, node_id: i32, new_metrics: SystemMetrics) {
        let mut system_metrics = self.system_metrics.lock().await;
        system_metrics.insert(node_id, new_metrics);
    }
}
