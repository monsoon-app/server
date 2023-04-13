use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};


use crate::service::monsoon::monsoon_service_server::MonsoonServiceServer;
use crate::service::Monsoon;

mod service;
mod server;
mod models;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "0.0.0.0:55443".parse().unwrap();
    let service = Monsoon::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(MonsoonServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
