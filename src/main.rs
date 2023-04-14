use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use crate::models::game::Game;


use crate::service::monsoon::monsoon_service_server::MonsoonServiceServer;
use crate::service::Monsoon;

mod service;
mod models;


lazy_static! {
    static ref GAMES: Arc<RwLock<HashMap<String, Game>>> = {
        let mut games_list = HashMap::new();
        Arc::new(RwLock::new(games_list))
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let games:Vec<Game>; // idk what to do about this, maybe like one global server thingy that service.rs can read/write to??

    let address = "0.0.0.0:55443".parse().unwrap();
    let service = Monsoon::default();
    println!("Server listening on {}", address);

    Server::builder()
        .add_service(MonsoonServiceServer::new(service))
        .serve(address)
        .await?;
    Ok(())
}
