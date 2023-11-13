mod event;
mod server;
mod client;
const AUTHOR: &str = "cowboy8625";

use clap::{crate_name, crate_version, Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    author = AUTHOR,
    about = "control two pc with one pc as if it was one",
    long_about = None
    )]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Server {
        #[arg(
            long,
            short = 'H',
            value_name = "HOST",
            default_value_t = String::from("127.0.0.1"),
        )]
        host: String,
        #[arg(
            long,
            short,
            value_name = "PORT",
            default_value_t = String::from("9895"),
        )]
        port: String,
    },
    Client {
        #[arg(
            long,
            short = 'H',
            value_name = "HOST",
            default_value_t = String::from("127.0.0.1"),
        )]
        host: String,
        #[arg(
            long,
            short,
            value_name = "PORT",
            default_value_t = String::from("9895"),
        )]
        port: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Server { host, port } => {
            server::spawn(host.clone(), port.clone()).await;
        }
        Commands::Client { host, port } => {
            client::spawn(host.clone(), port.clone()).await;
        }
    }
}
