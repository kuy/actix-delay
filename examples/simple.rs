extern crate actix_delay;

use actix_web::{web::resource, App, HttpServer};
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_delay::middleware::Delay::new(5000))
            .service(resource("/").to(|| async { "Delay" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
