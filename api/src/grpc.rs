use core::{Metrics, NodeInfo, RaftStatus};
use services::MasterNode;
use tonic::{Request, Response, Status};

pub struct GrpcServer {
    master: MasterNode,
}

impl GrpcServer {
    pub fn new(master: MasterNode) -> Self {
        Self { master }
    }

    pub async fn start(&self, addr: String) -> Result<(), Box<dyn std::error::Error>> {
        let server = tonic::transport::Server::builder()
            .add_service(DistributedSystemServer::new(self.master.clone()))
            .serve(addr.parse()?);
        server.await?;
        Ok(())
    }
}
