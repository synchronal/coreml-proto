use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Input: proto files from coreml-proto
  let proto_dir = &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
    .parent()
    .unwrap()
    .join("coremltools/mlmodel/format");

  // Output: generate directly into coreml-proto's source tree
  let output_dir = &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
    .parent()
    .unwrap()
    .join("coreml-proto/src/proto");

  // Find all proto files
  let proto_files: Vec<PathBuf> = std::fs::read_dir(proto_dir.to_str().unwrap())?
    .filter_map(|entry| entry.ok())
    .filter(|entry| {
      entry
        .path()
        .extension()
        .map(|ext| ext == "proto")
        .unwrap_or(false)
    })
    .map(|entry| entry.path())
    .collect();

  if proto_files.is_empty() {
    panic!("No .proto files found in {}", proto_dir.display());
  }

  println!("Found {} proto files to compile", proto_files.len());

  // Create output directory if it doesn't exist
  std::fs::create_dir_all(output_dir)?;

  // Configure prost-build to output to the coreml-proto src directory
  prost_build::Config::new()
    .out_dir(output_dir)
    .protoc_arg("--experimental_allow_proto3_optional")
    // Add clippy allow attributes to generated code
    .type_attribute(".", "#[allow(clippy::all)]")
    .type_attribute(".", "#[allow(clippy::pedantic)]")
    .type_attribute(".", "#[allow(clippy::nursery)]")
    // Compile all proto files
    .compile_protos(&proto_files, &[proto_dir])?;

  // Tell cargo to rerun if proto files change
  println!("cargo:rerun-if-changed={}", proto_dir.display());

  Ok(())
}
