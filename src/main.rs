use rgate::RgServer;

mod rgate;
mod upstream;
mod http;
mod config;

#[tokio::main]
async fn main() {
    let server = RgServer::from_default().await;
    server.run().await;
}
