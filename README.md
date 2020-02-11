# actix-delay

[![crates.io](https://img.shields.io/crates/v/actix-delay.svg)](https://crates.io/crates/actix-delay)
[![build](https://github.com/kuy/actix-delay/workflows/build/badge.svg)](https://github.com/kuy/actix-delay/actions)

Simulates a delayed response for [actix-web](https://actix.rs/).

This crate provides a middleware of actix-web which simulates a delayed/slow response for testing purpose.

## Usage

Add `actix-delay` to `dependencies` section in your project.

```
[dependencies]
actix-delay = "0.1"
```

Add `actix_delay::middleware::Delay` as middleware.

```
use actix_delay::middleware::Delay;

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Delay::new(3000))
            .service(resource("/").to(|| async { "Hello!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

See [full example](https://github.com/kuy/actix-delay/blob/master/examples/simple.rs).

## Examples

- [simple](https://github.com/kuy/actix-delay/blob/master/examples/simple.rs)
  - `cargo run --example simple`

## License

MIT

## Author

Yuki Kodama / [@kuy](https://twitter.com/kuy)
