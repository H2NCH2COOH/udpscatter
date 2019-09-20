use crate::crypto;

pub struct Server {}

pub fn new(
    listen_addr: &String,
    listen_port_range: &[String],
    target_addr: &String,
    target_port: u16,
    crypto_ctx: &Option<crypto::Ctx>,
) -> Result<Server, &'static str> {
    Ok(Server {})
}

impl Server {
    pub fn run(self) {}
}
