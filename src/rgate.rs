use tokio::net::{TcpStream, TcpListener};
use crate::config::{RgConfigProvider, self};

pub struct RgServer {
    listener: TcpListener,
    config_provider: RgConfigProvider,
}

struct RgClientAgent {

}

pub struct RgClient {
    tcp_stream: TcpStream,
    manager: RgClientAgent,
}

impl RgClient {
    fn from_stream(tcp_stream: TcpStream) -> RgClient {
        RgClient { 
            tcp_stream, 
            manager: RgClientAgent{} 
        }
    }

    async fn serve(&self) {

    }

}

impl RgServer {
    pub async fn from_addr_port(listen_addr: &str, listen_port: u16) -> RgServer {
        let spec_addr = format!("{}:{}", listen_addr, listen_port);
        let listener = TcpListener::bind(spec_addr).await.unwrap();

        RgServer { 
            listener,
            config_provider: RgConfigProvider::new(),
        }
    }

    pub async fn from_default() -> RgServer {
        let config_provider = RgConfigProvider::new();
        RgServer::from_addr_port(config_provider.get_listen_addr().as_str(), config_provider.get_port()).await
    }

    pub async fn run(self) -> ! {
        loop {
            let (tcp_stream, _) = self.listener.accept().await.unwrap();

            tokio::spawn(async move {
                let client = RgClient::from_stream(tcp_stream);
                client.serve().await;
            });
        }
    }
}