use std::fmt::format;
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use crate::service::monsoon::monsoon_service_server::MonsoonServiceServer;
use crate::service::Monsoon;

pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:55443".parse().unwrap();
    let ping = Monsoon::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(MonsoonServiceServer::new(ping))
        .serve(addr)
        .await?;
    Ok(())
}
