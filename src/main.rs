use api::{start_grpc_server, start_rest_api};
use infrastructure::{AuthService, DnsResolver};
use services::{AgentNode, MasterNode};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Initialize master node
    let master = MasterNode::new(1, "master_node".to_string());

    // Initialize auth service
    let auth_service = AuthService::new("your_secret_key".to_string());

    // Initialize DNS resolver
    let _dns_resolver = DnsResolver::new();

    // Start REST API on port 8330
    tokio::spawn(async move {
        start_rest_api(master.clone(), auth_service.clone(), 8330).await;
    });

    // Start gRPC server on port 8338
    tokio::spawn(async move {
        start_grpc_server(master.clone(), "0.0.0.0:8338".to_string())
            .await
            .unwrap();
    });

    // Wait for all tasks to complete
    tokio::signal::ctrl_c().await.unwrap();
}
