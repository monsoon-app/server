use std::fmt::format;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use monsoon::{PingRequest, PongResponse, Heartbeat, Storm, ConnectionResponse, HostRequest, HostResponse, StartRequest, StartResponse, PlayersResponse};
use monsoon::monsoon_service_server::MonsoonService;
use crate::{Game, GAMES};
use crate::models::location::Location;
use crate::models::player::Player;

pub mod monsoon {
    tonic::include_proto!("monsoon");
}

#[derive(Default)]
pub struct Monsoon {}

#[tonic::async_trait]
impl MonsoonService for Monsoon {
    async fn ping(&self, request:Request<PingRequest>) -> Result<Response<PongResponse>, Status> {
        println!("Received ping: {}", request.into_inner().content);
        Ok(Response::new(PongResponse {
            content: "Acknowledged.".to_string(),
        }))
    }

    async fn join(&self, request:Request<Heartbeat>) -> Result<Response<ConnectionResponse>, Status> {
        println!("_join");
        // todo: maybe wanna validate this stuff too
        let mut map = GAMES.write().unwrap();
        let r = &request.into_inner();
        if let Some(game) = map.get_mut(&r.code) {
            game.add_player(Player::new(
                r.name.to_string(),
                Location::new(r.latitude, r.longitude),
                0, // hidden
                0, // hider
            ));
            Ok(Response::new(ConnectionResponse {
                success: true,
            }))
        } else {
            Ok(Response::new(ConnectionResponse {
                success: false,
            }))
        }
    }

    async fn host(&self, request:Request<HostRequest>) -> Result<Response<HostResponse>, Status> {
        println!("_host");
        // todo: everything the client gives us here needs to be validated but we can do that later ;)
        let mut map = GAMES.write().unwrap();
        let mut rng = rand::thread_rng();
        let r = &request.into_inner();

        // generate lobby code
        let mut code = "MTSGA".to_string(); // MSTGA! easter egg 25% chance
        if map.contains_key(&code) || !(rng.gen_range(0..4) == 0) {
            code = "".to_string();
            let letters = "BCDFGHJKLMNPQRSTVWXYZ";
            loop {
                for _ in 0..5 {
                    let idx = rng.gen_range(0..letters.len());
                    code.push(letters.chars().nth(idx).unwrap());
                }
                if !map.contains_key(&code) {
                    break;
                }
                code.clear();
            }
        }

        let location = Location::new(r.latitude, r.longitude);
        map.insert(code.clone(), Game::new(
            code.clone(),
            r.lobby.to_string(), // validate later
            location.clone(),
            r.size, // validate later
            r.speed, // validate later
            0,
            0,
            vec![
                Player::new(
                    r.name.to_string(), // validate later
                    location,
                    1, // out
                    1, // hunter
                )
            ]
        ));
        Ok(Response::new(HostResponse {
            success: true,
            code,
        }))
    }

    async fn start(&self, request:Request<StartRequest>) -> Result<Response<StartResponse>, Status> {
        println!("_start");
        // todo: hi
        let mut map = GAMES.write().unwrap();
        let r = &request.into_inner();
        if let Some(game) = map.get_mut(&r.code) {
            game.start();
            Game::tick(&r.code);
            Ok(Response::new(StartResponse {
                success: true,
            }))
        } else {
            Ok(Response::new(StartResponse {
                success: false,
            }))
        }
    }

    async fn heartbeat(&self, request:Request<Heartbeat>) -> Result<Response<Storm>, Status> {
        println!("_heartbeat");
        let mut map = GAMES.write().unwrap();
        let r = &request.into_inner();
        if let Some(game) = map.get_mut(&r.code.to_string()) {
            if let Some(player) = game.get_player(&r.name.to_string()) {
                player.update(r.latitude, r.longitude)
            }
            Ok(Response::new(Storm {
                state: game.phase as i32,
                size: game.size,
                speed: game.speed,
                latitude: game.center.latitude,
                longitude: game.center.longitude,
            }))
        } else {
            Ok(Response::new(Storm {
                state: 3,
                size: 0.0,
                speed: 0.0,
                latitude: 0.0,
                longitude: 0.0,
            }))
        }
    }

    async fn get_players(&self, request:Request<Heartbeat>) -> Result<Response<PlayersResponse>, Status> {
        println!("_get_players");
        let players = vec![/* we can implement this later, ill just leave it empty for now */];
        let response = PlayersResponse { players };
        Ok(Response::new(response))
    }

}