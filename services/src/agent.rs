use distr_core::{Metrics, SystemMetrics, SystemMetricsCollector};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AgentNode {
    pub id: i32,
    pub name: String,
    pub node_info: NodeInfo,
    pub master_tx: mpsc::Sender<RaftStatus>,
    pub metrics_tx: mpsc::Sender<Metrics>,
    pub system_metrics_tx: mpsc::Sender<SystemMetrics>, // Новий канал для системних метрик
    pub metrics_collector: SystemMetricsCollector,      // Колектор метрик
}

impl AgentNode {
    pub fn new(
        id: i32,
        name: String,
        node_info: NodeInfo,
        master_tx: mpsc::Sender<RaftStatus>,
        metrics_tx: mpsc::Sender<Metrics>,
        system_metrics_tx: mpsc::Sender<SystemMetrics>, // Новий параметр
    ) -> Self {
        Self {
            id,
            name,
            node_info,
            master_tx,
            metrics_tx,
            system_metrics_tx,
            metrics_collector: SystemMetricsCollector::new(),
        }
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