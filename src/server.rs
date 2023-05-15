


    use tonic::{transport::Server, Request, Response, Status};
    use hello::say_server::{Say, SayServer};
    use hello::{KResponse, KRequest};

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
            Ok(Response::new(KResponse{ message:format!("pedido registado no nodo: {}",request.get_ref().reqdata), response_id: 0, response_type: 0, response_data: "".to_string(), request_knode_ori_id: 0 }))
            // reading data from request which is awrapper around our SayRequest message defined in .proto



        }
    }

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {

        // defining address for our service
        let addr = "10.0.2.6:9000".parse().unwrap();

        // creating a service
        let say = MySay::default();
        println!("Server listening on {}", addr);

        // adding our service to our server.
        Server::builder()
            .add_service(SayServer::new(say))
            .serve(addr)
            .await?;
        Ok(())
    }




    /* com segurança na comunicacao - tls

    from: https://docs.rs/tonic/latest/tonic/transport/index.html#traits

    
    let cert = std::fs::read_to_string("server.pem")?;
    let key = std::fs::read_to_string("server.key")?;

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .tls_config(ServerTlsConfig::new()
            .identity(Identity::from_pem(&cert, &key)))?
        .concurrency_limit_per_connection(256)
        .add_service(my_svc)
        .serve(addr)
        .await?;
    */
