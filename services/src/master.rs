use distr_core::{Metrics, NodeInfo, RaftStatus, SystemMetrics};
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
    pub system_metrics: Arc<Mutex<HashMap<i32, SystemMetrics>>>, // Нове поле
}

impl MasterNode {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            nodes: Arc::new(Mutex::new(HashMap::new())),
            raft_statuses: Arc::new(Mutex::new(Vec::new())),
            metrics: Arc::new(Mutex::new(HashMap::new())),
            system_metrics: Arc::new(Mutex::new(HashMap::new())), // Ініціалізація
        }
    }

    pub async fn update_system_metrics(&self, node_id: i32, new_metrics: SystemMetrics) {
        let mut system_metrics = self.system_metrics.lock().await;
        system_metrics.insert(node_id, new_metrics);
    }
}