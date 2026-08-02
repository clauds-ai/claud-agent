use distr_core::{NodeInfo, RaftStatus, SystemMetrics, SystemMetricsCollector};
use tokio::sync::mpsc;
use std::time::Duration;

#[derive(Debug)]
pub struct AgentNode {
    pub id: i32,
    pub name: String,
    pub node_info: NodeInfo,
    pub master_tx: mpsc::Sender<RaftStatus>,
    pub system_metrics_tx: mpsc::Sender<SystemMetrics>,
    pub metrics_collector: SystemMetricsCollector,
}

impl AgentNode {
    pub fn new(
        id: i32,
        name: String,
        node_info: NodeInfo,
        master_tx: mpsc::Sender<RaftStatus>,
        system_metrics_tx: mpsc::Sender<SystemMetrics>,
    ) -> Self {
        Self {
            id,
            name,
            node_info,
            master_tx,
            system_metrics_tx,
            metrics_collector: SystemMetricsCollector::new(),
        }
    }

    pub async fn send_raft_status(&self, status: RaftStatus) -> Result<(), mpsc::error::SendError<RaftStatus>> {
        self.master_tx.send(status).await
    }

    pub async fn send_system_metrics(&mut self) -> Result<(), mpsc::error::SendError<SystemMetrics>> {
        let system_metrics = self.metrics_collector.collect();
        self.system_metrics_tx.send(system_metrics).await
    }

    pub async fn start_monitoring(&mut self, interval_sec: u64) {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_sec));
        loop {
            interval.tick().await;
            let _ = self.send_system_metrics().await;
        }
    }
}
