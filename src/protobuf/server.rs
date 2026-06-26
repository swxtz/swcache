use tonic::{Request, Response, Status};

pub mod ping_proto {
    include!(concat!(env!("OUT_DIR"), "/ping.rs"));
}

use ping_proto::{
    ping_server::{Ping, PingServer},
    PingRequest, PingResponse,
};

pub struct PingServiceImpl;

#[tonic::async_trait]
impl Ping for PingServiceImpl {
    async fn send_ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        println!("Recebeu Ping: {:?}", request);

        let response = PingResponse {
            pong: "pong".to_string(),
        };

        Ok(Response::new(response))
    }
}

pub fn ping_service() -> PingServer<PingServiceImpl> {
    PingServer::new(PingServiceImpl)
}
