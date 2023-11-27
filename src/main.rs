

use std::env;


use dotenv::dotenv;
mod my_money_server;
pub mod storage;
// pub mod storage;
use log::info;

use crate::my_money_server::MyMoneyServer;
use tonic::transport::Server;
use my_money_server::money_server::MoneyServer;

use ::clap::Parser;
#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo - a simple CLI to send messages to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "6667")]
    port: u16,
    /// The message to send
    message: String,
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init();
    
    let server_addr = env::var("SERVER_ADDR").expect("Can't get DB URL");
    // let server_url = env::var("SERVER_PORT").expect("Can't get DB URL");
    let addr=server_addr.parse().expect("Invalid addres");
    info!("Starting Server on {server_addr}");

    let haha = MyMoneyServer::default();
    Server::builder()
        .add_service(MoneyServer::new(haha))
        .serve(addr)
        .await?;

    Ok(())
}