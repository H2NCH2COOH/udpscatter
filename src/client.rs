use crate::crypto;
use crate::utils;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Mutex, RwLock};
use tokio::net::UdpSocket;
use tokio::prelude::*;

pub struct Client<'a> {
    crypto_ctx: Option<&'a crypto::Ctx<'a>>,
    listen_sock: UdpSocket,
    server_addr: IpAddr,
    server_ports: Vec<u16>,
    client_socks: Vec<UdpSocket>,
    peer_addr: RwLock<Option<SocketAddr>>,
    tx_buff: Mutex<[u8; 2000]>,
    rx_buff: Mutex<[u8; 2000]>,
}

pub fn new<'a>(
    listen_addr: &String,
    listen_port: u16,
    server_addr: &String,
    server_port_range: &[String],
    crypto_ctx: Option<&'a crypto::Ctx>,
) -> Result<Client<'a>, &'static str> {
    let ipaddr: IpAddr = listen_addr.parse().map_err(|_e| "Invalid listen address")?;
    if listen_port == 0 {
        return Err("Invalid listen port");
    }
    let sockaddr = SocketAddr::new(ipaddr, listen_port);
    let listen_sock = UdpSocket::bind(&sockaddr).map_err(|_e| "Failed to create listen socket")?;

    let server_addr: IpAddr = server_addr.parse().map_err(|_e| "Invalid server address")?;
    let server_ports = utils::compile_port_range(server_port_range)?;

    let bind_addr = if server_addr.is_ipv4() {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
    } else {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 0)
    };
    let client_socks: Vec<UdpSocket> = server_ports
        .iter()
        .map(|_p| UdpSocket::bind(&bind_addr).map_err(|_e| "Failed to create client socket"))
        .collect::<Result<_, _>>()?;

    let peer_addr = RwLock::new(None);

    let tx_buff = Mutex::new([0u8; 2000]);
    let rx_buff = Mutex::new([0u8; 2000]);

    Ok(Client {
        crypto_ctx,
        listen_sock,
        server_addr,
        server_ports,
        client_socks,
        peer_addr,
        tx_buff,
        rx_buff,
    })
}

impl Client<'_> {
    pub fn run(self) {
        //tokio::run(future::lazy(|| self.start_listen()));
    }

    fn f(&self) -> impl Future {
        let mut buff = [0u8; 2000];
        self.listen_sock
            .recv_dgram(buff.as_mut())
            .and_then(|(_, buff, len, peer_addr)| {
                let mut p = self.peer_addr.write().unwrap();
                *p = Some(peer_addr);

                self.f()
            })
    }
}
