// Declare the Protobuf-generated module
mod distributed_system; // <-- Add this line

pub mod grpc;
pub mod rest;

pub use grpc::start_grpc_server;
pub use rest::start_rest_api;
