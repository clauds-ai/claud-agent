use crate::node::NodeInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaftStatusEnum {
    Request,
    Sync,
    Response,
    Metrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftStatus {
    pub status: RaftStatusEnum,
    pub message: String,
    pub node: NodeInfo,
}

impl RaftStatus {
    pub fn new(status: RaftStatusEnum, message: String, node: NodeInfo) -> Self {
        Self {
            status,
            message,
            node,
        }
    }
}
