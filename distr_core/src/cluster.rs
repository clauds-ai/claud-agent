use crate::node::NodeInfo;
use serde::{Deserialize, Serialize}; // Import `NodeInfo` from the `node` module

/// Represents information about the entire cluster, including all nodes and the master node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    /// List of all nodes in the cluster.
    pub nodes: Vec<NodeInfo>,
    /// The master node of the cluster.
    pub master: NodeInfo,
    /// Name of the cluster.
    pub cluster_name: String,
}

impl ClusterInfo {
    /// Creates a new `ClusterInfo` instance.
    pub fn new(nodes: Vec<NodeInfo>, master: NodeInfo, cluster_name: String) -> Self {
        Self {
            nodes,
            master,
            cluster_name,
        }
    }
}
