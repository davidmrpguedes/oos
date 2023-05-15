

use std;

use protobuf::well_known_types::Value_oneof_kind::string_value;
use tonic::{transport::Server, Request, Response, Status};
use crate::grpc::string::String;

// para o server:
use hello::say_server::{Say, SayServer};
use hello::{KResponse, KRequest};
use hello::k_request;
use hello::k_response;

// para o client:
use hello::say_client::SayClient;

// log
use simplelog::*;
use std::fs::File;
use std::string;
use rand::random;


mod hello;

// defining a struct for our service
#[derive(Default)]
pub struct MySay {}


// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MySay {
    // our rpc implemented as function
    async fn send(&self,request:Request<KRequest>)->Result<Response<KResponse>,Status>{

        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(
            KResponse{
                message:format!("pedido registado no nodo: {}",request.get_ref().reqdata),
                request_knode_ori_id: 0,
                response_id: 0,
                response_data: String::new(),
                response_type: 0
            }
        )
        )
        // reading data from request which is a wrapper around our SayRequest message defined in .proto

    }
}


#[tokio::main]
pub async fn server_init(ip_addr: String, port_num: String) -> Result<(), Box<dyn std::error::Error>> {

    // defining address for our service
    let addr = format!("{}:{}", ip_addr, port_num);
    println!("{}", addr);

    // creating a service
    let say = MySay::default();
    println!("Server listening on {}", addr);

    // adding our service to our server.
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr.parse().unwrap())
        .await?;
    Ok(())
}


#[tokio::main]
pub async unsafe fn snd_request(ip_addr: String, port_num: String, user_request: Krequest) -> Result<(), Box<dyn std::error::Error>> {

    // defining address for our service
    let mut addr_todo = format!("http://{}:{}", ip_addr, port_num);

    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::builder( addr_todo.parse().unwrap())
        .connect()
        .await?;

    // creating gRPC client from channel
    let mut client = SayClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(
        KRequest {
            reqdata: String::new(),
            request_id: random(),
            request_knode_id: 0, // TODO: tem de ser o id do node que envia.
            request_type: user_request.request_type,
            request_data: String::new()
        },
    );

    // sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
