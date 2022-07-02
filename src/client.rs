use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;


pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr:      "888888".to_owned(),
            amount: 1
        }
    );

    let response = client.send_payment(request).await?;

    println!("Response={:?}", response);
    Ok(())
}