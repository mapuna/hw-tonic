use tonic::{transport::Server, Request, Response, Status};

use hw::greeter_server::{Greeter, GreeterServer};
use hw::{HelloReply, HelloRequest};

pub mod hw {
    tonic::include_proto!("hw");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hw::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;
    Ok(())
}