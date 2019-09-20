use crate::crypto;
use crate::utils;
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;

pub struct Client {
    listen_sock: UdpSocket,
}

pub fn new(
    listen_addr: &String,
    listen_port: u16,
    server_addr: &String,
    server_port_range: &[String],
    crypto_ctx: &Option<crypto::Ctx>,
) -> Result<Client, &'static str> {
    let ipaddr: IpAddr = listen_addr.parse().map_err(|_e| "Invalid listen address")?;
    if listen_port == 0 {
        return Err("Invalid listen port");
    }
    let sockaddr = SocketAddr::new(ipaddr, listen_port);
    let listen_sock = UdpSocket::bind(&sockaddr).map_err(|_e| "Failed to create listen socket")?;

    let ipaddr: IpAddr = server_addr.parse().map_err(|_e| "Invalid server address")?;
    let ports = utils::compile_port_range(server_port_range)?;



    Ok(Client { listen_sock })
}

impl Client {
    pub fn run(self) {}
}
