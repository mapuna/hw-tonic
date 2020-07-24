use hw::greeter_client::GreeterClient;
use hw::HelloRequest;

pub mod hw {
    tonic::include_proto!("hw");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Anupam".into()
    });

    let response = client.say_hello(request).await?;
    println!("Response: {:?}", response);

    Ok(())
}