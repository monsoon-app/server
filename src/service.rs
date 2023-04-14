use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use monsoon::{PingRequest, PongResponse, Heartbeat, Storm, ConnectionResponse, HostRequest, HostResponse, StartRequest, StartResponse, PlayersResponse, Player};
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
        if map.contains_key(request.into_inner().code) {
            if let Some(game) = map.get_mut(code) {
                game.add_player(Player::new(
                    request.into_inner().name,
                    Location::new(request.into_inner().latitude, request.into_inner().longitude),
                    0, // hidden
                    0, // hider
                ))
            }
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
        let code = Game::generate_lobby_code(map.keys().map(|key| *key).collect());
        let location = Location::new(request.into_inner().latitude, request.into_inner().longitude);
        map.insert(code, Game::new(
            code.clone(),
            request.into_inner().lobby, // validate later
            location,
            request.into_inner().size, // validate later
            request.into_inner().speed, // validate later
            0,
            0,
            vec![
                Player::new(
                    request.into_inner().name, // validate later
                    location.clone(),
                    1, // out
                    1, // hunter
                )
            ]
        ));
        Ok(Response::new(HostResponse {
            success: true,
            code: code.clone(),
        }))
    }

    async fn start(&self, request:Request<StartRequest>) -> Result<Response<StartResponse>, Status> {
        println!("_start");
        // todo: hi
        let mut map = GAMES.write().unwrap();
        if map.contains_key(request.into_inner().code) {
            if let Some(game) = map.get_mut(code) {
                game.start()
            }
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
        Ok(Response::new(Storm {
            state: 0,
            size: 300.0,
            speed: 30.0,
            latitude: 40.070531,
            longitude: -75.450966,
        }))
    }

    async fn get_players(&self, request:Request<Heartbeat>) -> Result<Response<PlayersResponse>, Status> {
        println!("_get_players");
        let players = vec![/* we can implement this later, ill just leave it empty for now */];
        let response = PlayersResponse { players };
        Ok(Response::new(response))
    }

}