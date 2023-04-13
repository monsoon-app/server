use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use crate::models::game::Game;


use crate::service::monsoon::monsoon_service_server::MonsoonServiceServer;
use crate::service::Monsoon;

mod service;
mod models;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let games:Vec<Game>; // idk what to do about this, maybe like one global server thingy that service.rs can read/write to??

    let address = "0.0.0.0:55443".parse().unwrap();
    let service = Monsoon::default();
    println!("Server listening on {}", address);

    Server::builder()
        .add_service(MonsoonServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
