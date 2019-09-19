mod client;
mod crypto;
mod server;
mod utils;

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Usage:
    udpscatter server [--key <key>] <listen-addr> <listen-port-range>...
    udpscatter client [--key <key>] <listen-addr> <listen-port> <server-addr> <server-port-range>...

Options:
    --key <key>         An optional key to encrypt the UDP packet
";

#[derive(Deserialize, Debug)]
struct Args {
    arg_listen_addr: String,
    arg_listen_port_range: Vec<String>,
    arg_listen_port: u16,
    arg_server_addr: String,
    arg_server_port_range: Vec<String>,
    flag_key: String,
    cmd_server: bool,
    cmd_client: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let mut crypto_ctx: Option<crypto::Ctx> = None;
    if args.flag_key.len() > 0 {
        crypto_ctx = Some(
            crypto::new_ctx(args.flag_key.as_str().as_bytes()).unwrap_or_else(|e| {
                eprintln!("Failed to create crypto context with error: {}", e);
                std::process::exit(1)
            }),
        );
    }

    if args.cmd_server {
        let server = server::new(
            &args.arg_listen_addr,
            &args.arg_listen_port_range,
            &crypto_ctx,
        )
        .unwrap_or_else(|e| {
            eprintln!("Failed to create server with error: {}", e);
            std::process::exit(1)
        });
        server.run();
    } else {
        let client = client::new(
            &args.arg_listen_addr,
            args.arg_listen_port,
            &args.arg_server_addr,
            &args.arg_server_port_range,
            &crypto_ctx,
        )
        .unwrap_or_else(|e| {
            eprintln!("Failed to create client with error: {}", e);
            std::process::exit(1)
        });
        client.run();
    }
}
