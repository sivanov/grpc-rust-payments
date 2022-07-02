fn main() -> Result<(), Box<dyn std::error::Error>> {
    // show to tonic where is proto description is located
    tonic_build::compile_protos("proto/payments.proto");
    Ok(())
}