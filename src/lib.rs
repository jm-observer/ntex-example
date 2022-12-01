use ntex::connect::openssl::Connector;
use ntex::connect::Address;
use ntex::time::Seconds;
use ntex_mqtt::v5;
use openssl::ssl;

#[derive(Debug)]
struct Error;

impl std::convert::TryFrom<Error> for v5::PublishAck {
    type Error = Error;

    fn try_from(err: Error) -> Result<Self, Self::Error> {
        Err(err)
    }
}

pub async fn connet<A>(addr: A) -> std::io::Result<()>
where
    A: Address + Clone,
{
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    // ssl connector
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls()).unwrap();
    builder.set_verify(ssl::SslVerifyMode::PEER);
    builder
        .set_ca_file("./certs/broker.emqx.io-ca.crt")
        .unwrap();
    // connect to server
    let _client = v5::client::MqttConnector::new(addr)
        .connector(Connector::new(builder.build()))
        .client_id("user")
        .keep_alive(Seconds::ONE)
        .connect()
        .await
        .unwrap();

    Ok(())
}
