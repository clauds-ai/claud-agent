pub mod agent;
pub mod master;
pub mod systemd;

pub use agent::AgentNode;
pub use master::MasterNode;

#[cfg(test)]
mod tests {
    use super::*;
    use distr_core::{NodeInfo, RaftStatus};
    use distr_core::raft::RaftStatusEnum;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_master_node() {
        let master = MasterNode::new(1, "test_master".to_string());
        let node = NodeInfo::new(
            1,
            "test_node".to_string(),
            "test_position".to_string(),
            "test_signal".to_string(),
        );
        master.add_node(node).await;
        assert_eq!(master.nodes.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn test_agent_node() {
        let (tx, mut rx) = mpsc::channel(10);
        let agent = AgentNode::new(
            1,
            "test_agent".to_string(),
            NodeInfo::new(
                1,
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            ),
            tx,
        );
        let status = RaftStatus::new(
            RaftStatusEnum::Request,
            "test".to_string(),
            NodeInfo::new(
                1,
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            ),
        );
        agent.send_raft_status(status).await.unwrap();
        assert!(rx.recv().await.is_some());
    }
}
