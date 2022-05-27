use std::fs;

use actix_web::{App, HttpServer};
use actix_files::Files;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};



#[tokio::main]
async fn main() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    //let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    //builder
    //    .set_private_key_file("key.pem", SslFiletype::PEM)
    //    .unwrap();
    //builder.set_certificate_chain_file("cert.pem").unwrap();

    println!("test");

    HttpServer::new(|| {
        App::new()
            //.route("/", fs::read_to_string("./www/index.html").unwrap())
            .service(Files::new("/", "./www").prefer_utf8(true))   
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}