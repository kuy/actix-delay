extern crate actix_delay;

use actix_delay::middleware::Delay;
use actix_web::{web::resource, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Delay::new(5000))
            .service(resource("/").to(|| async { "Delay" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
