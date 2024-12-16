use log;

use actix_web::{middleware::Logger, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Ping(msg)) => session.pong(msg).await.unwrap(),
                Ok(AggregatedMessage::Text(text)) => session.text(text).await.unwrap(),
                Ok(AggregatedMessage::Binary(bin)) => session.binary(bin).await.unwrap(),
                _ => (),
            }
        }
    });

    Ok(res)
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
            let logger = Logger::default();

            App::new().route("/ws/echo", web::get().to(echo)).wrap(logger).service(index)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
