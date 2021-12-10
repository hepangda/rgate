use tokio::net::TcpListener;

const RGATE_HTTP_PORT: u16 = 80;
const RGATE_HTTPS_PORT: u16 = 443;
const RGATE_DEFAULT_PORT: u16 = RGATE_HTTP_PORT;


pub struct RgateServer {
    listener: TcpListener,
}

pub struct RgateClient {

}

impl RgateServer {
    async fn new(listen_addr: String, listen_port: u16) -> RgateServer {
        let spec_addr = format!("{}:{}", listen_addr, listen_port);
        let listener = TcpListener::bind(spec_addr).await.unwrap();

        RgateServer { listener }
    }

    async fn run(self) -> ! {
        loop {
            let (mut stream, _) = self.listener.accept().await.unwrap();

            tokio::spawn(async move {
                // TODO: 
            });
        }
    }
}