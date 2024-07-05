use tonic::{transport::Server, Request, Response};
use crate::check_example::HelloRequest;
use crate::check_example::check_server::{Check,CheckServer};

pub mod check_example {
    tonic::include_proto!("example");
}

#[derive(Debug, Default)]
pub struct MyCheck {}

#[tonic::async_trait]
impl Check for MyCheck {

    async fn check_request(&self, request: Request<HelloRequest>,)->Result<Response<check_example::HelloResponse>, tonic::Status>
    {
        let req=request.into_inner();
        let reply=check_example::HelloResponse {
             id:req.id,
             message:format!("id is {} and Name is {} ",req.id,req.name)
        };
           Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>{
    let address="[::1]:50051".parse()?;
    let check_service=MyCheck::default();
    println!("Listening...");
    Server::builder()
        .add_service(CheckServer::new(check_service))
        .serve(address).await?;
    Ok(())
}
