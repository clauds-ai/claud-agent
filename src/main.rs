use api::{start_grpc_server, start_rest_api};
use infrastructure::{AuthService, DnsResolver};
use services::{AgentNode, MasterNode};
use distr_core::{NodeInfo, SystemMetrics};
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

    // Create a channel for system metrics
    let (system_metrics_tx, mut system_metrics_rx) = tokio::sync::mpsc::channel(100);

    // Create an agent node
    let agent = AgentNode::new(
        2,
        "agent_node".to_string(),
        NodeInfo::new(2, "agent_node".to_string(), "agent".to_string(), "".to_string()),
        master.raft_statuses_tx(), // Припускаємо, що такий метод існує
        master.metrics_tx(),        // Припускаємо, що такий метод існує
        system_metrics_tx,
    );

    // Start monitoring system metrics
    let mut agent_clone = agent.clone();
    tokio::spawn(async move {
        agent_clone.start_monitoring(5).await; // Збір метрик кожні 5 секунд
    });

    // Process received system metrics
    let master_clone = master.clone();
    tokio::spawn(async move {
        while let Some(metrics) = system_metrics_rx.recv().await {
            master_clone.update_system_metrics(2, metrics).await;
        }
    });

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
