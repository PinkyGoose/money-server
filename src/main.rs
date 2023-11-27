

use std::env;


use dotenv::dotenv;
mod my_money_server;
pub mod storage;
// pub mod storage;
use log::info;

use crate::my_money_server::MyMoneyServer;
use tonic::transport::Server;
use my_money_server::money_server::MoneyServer;


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