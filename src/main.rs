mod client;
mod crypto;
mod server;
mod utils;

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &str = "
Usage:
    udpscatter server [--key <key>] <target-addr> <target-port> <listen-addr> <listen-port-range>...
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
    arg_target_addr: String,
    arg_target_port: u16,
    flag_key: String,
    cmd_server: bool,
    cmd_client: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let crypto_ctx: Option<crypto::Ctx> = if args.flag_key.is_empty() {
        None
    } else {
        Some(
            crypto::new_ctx(args.flag_key.as_str().as_bytes()).unwrap_or_else(|e| {
                eprintln!("Failed to create crypto context with error: {}", e);
                std::process::exit(1)
            }),
        )
    };

    if args.cmd_server {
        let server = server::new(
            &args.arg_listen_addr,
            &args.arg_listen_port_range,
            &args.arg_target_addr,
            args.arg_target_port,
            crypto_ctx.as_ref(),
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
            crypto_ctx.as_ref(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Failed to create client with error: {}", e);
            std::process::exit(1)
        });

        client.run();
    }
}
