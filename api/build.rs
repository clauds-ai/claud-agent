fn main() {
    // Generate Protobuf code
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile(&["../proto/distributed_system.proto"], &["../proto/"])
        .unwrap_or_else(|e| panic!("Protobuf compilation failed: {}", e));

    // Post-process the generated file to fix `::core::` references
    let generated_file = "src/distributed_system.rs";
    let content = std::fs::read_to_string(generated_file).unwrap_or_else(|_| {
        panic!("Failed to read generated file: {}", generated_file);
    });

    // Replace `::core::` with `std::` to avoid name collision
    let fixed_content = content
        .replace("::core::option", "std::option")
        .replace("::core::result", "std::result")
        .replace("::core::default", "std::default")
        .replace("::core::fmt", "std::fmt")
        .replace("::core::convert", "std::convert")
        .replace("::core::marker", "std::marker")
        .replace("::core::cmp", "std::cmp");

    std::fs::write(generated_file, fixed_content).unwrap_or_else(|_| {
        panic!("Failed to write fixed content to: {}", generated_file);
    });
}
