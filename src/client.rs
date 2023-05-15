

use hello::say_client::SayClient;
use hello::KRequest;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// creating a channel ie connection to server
    // TODO : usa IP fixo neste momento...
    let channel = tonic::transport::Channel::from_static("http://10.0.2.6:9000")
    .connect()
    .await?;
// creating gRPC client from channel
    let mut client = SayClient::new(channel);
// creating a new Request
    let request = tonic::Request::new(
        KRequest {
           reqdata:String::from("teste lourenco"),
            request_knode_id: 0,   // TODO: tem de sero id do node que envia
            request_id: 0,
            request_type: 0,
            request_data: "".to_string(),
        },
    );
// sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}


/*
from: https://docs.rs/tonic/latest/tonic/transport/index.html#traits

let cert = std::fs::read_to_string("ca.pem")?;

let mut channel = Channel::from_static("https://example.com")
    .tls_config(ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(&cert))
        .domain_name("example.com".to_string()))?
    .timeout(Duration::from_secs(5))
    .rate_limit(5, Duration::from_secs(1))
    .concurrency_limit(256)
    .connect()
    .await?;

channel.call(Request::new(BoxBody::empty())).await?;
 */