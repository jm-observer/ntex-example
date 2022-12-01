use http::Uri;
use ntex_example::connet;
use std::str::FromStr;
#[ntex::main]
async fn main() {
    let addr = "broker-cn.emqx.io:8883";
    let uri = Uri::from_str(addr).unwrap();
    connet(uri).await.unwrap();
}
