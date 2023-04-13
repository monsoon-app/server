use std::fmt::format;
use futures::{SinkExt, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use monsoon::{PingRequest, PongResponse, Heartbeat, Storm, ConnectionResponse, HostRequest, HostResponse, PlayersResponse, Player};
use monsoon::monsoon_service_server::MonsoonService;

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
        Ok(Response::new(ConnectionResponse {
            success: false,
        }))
    }

    async fn host(&self, request:Request<HostRequest>) -> Result<Response<HostResponse>, Status> {
        println!("_host");
        Ok(Response::new(HostResponse {
            success: false,
            code: "MTSGA".to_string(),
        }))
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