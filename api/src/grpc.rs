// Declare the generated Protobuf module
mod distributed_system; // <-- Add this line

use core::{ClusterInfo, Metrics, NodeInfo, RaftStatus};
use distributed_system::distributed_system_server::DistributedSystem;
use distributed_system::{
    AuthToken, ClusterInfo as ProtoClusterInfo, Metrics as ProtoMetrics, NodeInfo as ProtoNodeInfo,
    RaftStatus as ProtoRaftStatus,
};
use services::MasterNode;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// gRPC service implementation for the distributed system.
#[derive(Debug, Default)]
pub struct GrpcService {
    pub master: Arc<MasterNode>,
}

#[tonic::async_trait]
impl DistributedSystem for GrpcService {
    async fn get_node_info(
        &self,
        request: Request<ProtoNodeInfo>,
    ) -> Result<Response<ProtoNodeInfo>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn send_raft_status(
        &self,
        request: Request<ProtoRaftStatus>,
    ) -> Result<Response<ProtoRaftStatus>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn send_metrics(
        &self,
        request: Request<ProtoMetrics>,
    ) -> Result<Response<ProtoMetrics>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn get_cluster_info(
        &self,
        _request: Request<AuthToken>,
    ) -> Result<Response<ProtoClusterInfo>, Status> {
        let nodes = self.master.nodes.lock().await;
        let master_node = NodeInfo::new(
            self.master.id,
            self.master.name.clone(),
            "master".to_string(),
            "".to_string(),
        );

        let cluster_info = ProtoClusterInfo {
            nodes: nodes
                .values()
                .map(|node| ProtoNodeInfo {
                    id: node.id,
                    name: node.name.clone(),
                    position: node.position.clone(),
                    signal_metrics: node.signal_metrics.clone(),
                    metadata: node.metadata.clone().into_iter().collect(),
                })
                .collect(),
            master: Some(master_node.into()),
            cluster_name: "distributed_system".to_string(),
        };

        Ok(Response::new(cluster_info))
    }
}

/// Starts the gRPC server for the distributed system.
pub async fn start_grpc_server(
    master: MasterNode,
    addr: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let grpc_service = GrpcService {
        master: Arc::new(master),
    };

    Server::builder()
        .add_service(DistributedSystemServer::new(grpc_service))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
