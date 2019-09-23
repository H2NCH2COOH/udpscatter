use crate::crypto;
use crate::utils;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use tokio::net::UdpSocket;
use tokio::prelude::*;

struct ClientCtx {
    server_addr: IpAddr,
    server_ports: Vec<u16>,
    crypto_ctx: Option<crypto::Ctx>,
    peer_addr: RwLock<Option<SocketAddr>>,
}

pub async fn run(
    listen_addr: &String,
    listen_port: u16,
    server_addr: &String,
    server_port_range: &[String],
    crypto_ctx: Option<crypto::Ctx>,
) -> Result<(), &'static str> {
    let ipaddr: IpAddr = listen_addr.parse().map_err(|_e| "Invalid listen address")?;
    if listen_port == 0 {
        return Err("Invalid listen port");
    }
    let sockaddr = SocketAddr::new(ipaddr, listen_port);
    let listen_sock = UdpSocket::bind(&sockaddr)
        .await
        .map_err(|_e| "Failed to create listen socket")?;

    let server_addr: IpAddr = server_addr.parse().map_err(|_e| "Invalid server address")?;
    let server_ports = utils::compile_port_range(server_port_range)?;

    let bind_addr = if server_addr.is_ipv4() {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
    } else {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 0)
    };

    let peer_addr = RwLock::new(None);

    let ctx = Arc::new(ClientCtx {
        server_addr: server_addr,
        server_ports: server_ports,
        crypto_ctx: crypto_ctx,
        peer_addr: peer_addr,
    });

    for _ in 0..ctx.server_ports.len() {
        let s = UdpSocket::bind(&bind_addr)
            .await
            .map_err(|_e| "Failed to create client socket")?;
        tokio::spawn(run_client_socket(ctx.clone(), s));
    }

    run_listen_socket(ctx, listen_sock).await;

    Ok(())
}

async fn run_client_socket(ctx: Arc<ClientCtx>, mut sock: UdpSocket) {
    loop {
        let mut buff = [0u8; 2000];
        let rst = sock.recv_from(&mut buff).await;
        match rst {
            Ok((len, _)) => {
                if let Some(c) = ctx.crypto_ctx {
                    c.decrypt(&buff[0..len]);
                }

            }
            Err(e) => {
                eprintln!(
                    "Failed to receive packet from client socket with error: {}",
                    e
                );
                break;
            }
        }
    }
}

async fn run_listen_socket(ctx: Arc<ClientCtx>, mut sock: UdpSocket) {
    loop {
        let mut buff = [0u8; 2000];
        let rst = sock.recv_from(&mut buff).await;
        match rst {
            Ok((len, peer_addr)) => {
                let mut p = ctx.peer_addr.write().unwrap();
                *p = Some(peer_addr);
                //send_to_server(ctx, buff, len).await;
            }
            Err(e) => {
                eprintln!(
                    "Failed to receive packet from listening socket with error: {}",
                    e
                );
                break;
            }
        }
    }
}
