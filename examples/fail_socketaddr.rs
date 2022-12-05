use ntex_example::connet;
use std::net::SocketAddr;
use std::str::FromStr;

#[ntex::main]
async fn main() {
    let addr = "3.228.54.173:8883";
    let addr: SocketAddr = SocketAddr::from_str(addr).unwrap();
    connet(addr).await.unwrap();
}
