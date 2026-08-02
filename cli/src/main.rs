use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::Value;

#[derive(Parser)]
#[command(name = "cluster-cli")]
#[command(about = "CLI tool to interact with the distributed system cluster")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get cluster information")]
    GetClusterInfo { token: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::GetClusterInfo { token } => {
            let response = client
                .get("http://localhost:8330/cluster/info")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap();
            let cluster_info: Value = response.json().await.unwrap();
            println!("{:#?}", cluster_info);
        }
    }
}
