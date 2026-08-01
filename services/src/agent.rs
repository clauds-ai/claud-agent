use core::{Metrics, NodeInfo, RaftStatus};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AgentNode {
    pub id: i32,
    pub name: String,
    pub node_info: NodeInfo,
    pub master_tx: mpsc::Sender<RaftStatus>,
    pub metrics_tx: mpsc::Sender<Metrics>,
}

impl AgentNode {
    pub fn new(
        id: i32,
        name: String,
        node_info: NodeInfo,
        master_tx: mpsc::Sender<RaftStatus>,
        metrics_tx: mpsc::Sender<Metrics>,
    ) -> Self {
        Self {
            id,
            name,
            node_info,
            master_tx,
            metrics_tx,
        }
    }

    pub async fn send_raft_status(
        &self,
        status: RaftStatus,
    ) -> Result<(), mpsc::error::SendError<RaftStatus>> {
        self.master_tx.send(status).await
    }

    pub async fn send_metrics(
        &self,
        metrics: Metrics,
    ) -> Result<(), mpsc::error::SendError<Metrics>> {
        self.metrics_tx.send(metrics).await
    }
}
