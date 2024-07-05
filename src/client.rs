use check_example::{check_client::CheckClient, HelloRequest};

pub mod check_example {
    tonic::include_proto!("example");
}
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>{

    let mut client=CheckClient::connect("http://[::1]:50051").await?;

    let requests = vec![
        HelloRequest{id:1, name:"Kumar".into()},
        HelloRequest{id:2, name:"Raj".into()},
        HelloRequest{id:33, name:"Singh".into()}
    ];
    for request in requests{
        let requests=tonic::Request::new(request);
        
    

    // let request=tonic::Request::new(HelloRequest{
    //     id:23.into(),
    //     name:"Kumar".into()
    // });

    let response=client.check_request(requests).await?;
    println!("Response = {:?}",response.into_inner());
    }
    Ok(())
}