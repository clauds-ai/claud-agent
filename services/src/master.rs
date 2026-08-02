use distr_core::{Metrics, NodeInfo, RaftStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct MasterNode {
    pub id: i32,
    pub name: String,
    pub nodes: Arc<Mutex<HashMap<i32, NodeInfo>>>,
    pub raft_statuses: Arc<Mutex<Vec<RaftStatus>>>,
    pub metrics: Arc<Mutex<HashMap<i32, Metrics>>>,
}

impl MasterNode {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            nodes: Arc::new(Mutex::new(HashMap::new())),
            raft_statuses: Arc::new(Mutex::new(Vec::new())),
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node: NodeInfo) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        if nodes.contains_key(&node.id) {
            return Err(format!("Node with ID {} already exists", node.id));
        }
        nodes.insert(node.id, node);
        Ok(())
    }

    pub async fn update_raft_status(&self, status: RaftStatus) {
        let mut raft_statuses = self.raft_statuses.lock().await;
        raft_statuses.push(status);
    }

    pub async fn update_metrics(&self, node_id: i32, new_metrics: Metrics) {
        let mut metrics = self.metrics.lock().await;
        metrics.insert(node_id, new_metrics);
    }
}
