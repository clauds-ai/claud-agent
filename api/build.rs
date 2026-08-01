fn main() {
    tonic_build::configure()
        .build_server(true) // Generate gRPC server code
        .out_dir("src/") // Output generated code to `api/src/`
        .compile(
            &["/Users/yevhen/Repos/claud-agent/proto/distributed_system.proto"], // Path to your `.proto` file (relative to `api/`)
            &["/Users/yevhen/Repos/claud-agent/proto/"], // Include directory for imports
        )
        .unwrap_or_else(|e| panic!("Protobuf compilation failed: {}", e));
}
