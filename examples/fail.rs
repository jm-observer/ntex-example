use ntex_example::connet;
#[ntex::main]
async fn main() {
    let addr = "broker-cn.emqx.io:8883";
    connet(addr).await.unwrap();
}
