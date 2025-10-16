fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile with tonic-build for gRPC support
    tonic_build::compile_protos("proto/stunnel.proto")?;

    println!("cargo:rerun-if-changed=proto/stunnel.proto");
    Ok(())
}