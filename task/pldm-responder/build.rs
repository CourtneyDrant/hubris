fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    idol::client::build_client_stub("../../idl/fw-device.idol", "client_stub.rs")?;

    Ok(())
}
