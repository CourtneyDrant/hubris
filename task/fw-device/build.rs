fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    idol::server::build_server_support(
        "../../idl/fw-device.idol",
        "server_stub.rs",
        idol::server::ServerStyle::InOrder,
    )?;

    Ok(())
}
