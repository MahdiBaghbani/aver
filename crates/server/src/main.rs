use poem::http::{Method, Uri};
use poem::{listener::TcpListener, Endpoint, EndpointExt, IntoResponse, Request, Response, Route, Server};
use poem_openapi::OpenApiService;
use std::time::{Duration, Instant};
use tracing::{error, info};

use crate::ocm::endpoints::Ocm;
mod settings;
mod ocm;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();


    let api_service =
        OpenApiService::new(Ocm, "Hello World", "1.0")
            .server("http://localhost:3000");
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/swag", ui)
        .around(log);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

async fn log<E: Endpoint>(next: E, req: Request) -> poem::Result<Response> {
    let method: Method = req.method().clone();
    let path: Uri = req.uri().clone();

    let start: Instant = Instant::now();
    let res = next.call(req).await;
    let elapsed: Duration = start.elapsed();

    match res {
        Ok(r) => {
            let resp = r.into_response();

            info!(
                "{} -> {} {} [ {:?} ] - {:?}",
                method.as_str(),
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or(""),
                elapsed,
                path.path(),
            );

            Ok(resp)
        }
        Err(e) => {
            let msg: String = format!("{}", &e);
            let resp: Response = e.into_response();

            if resp.status().as_u16() >= 500 {
                error!("{}", msg);
            }

            info!(
                "{} -> {} {} [ {:?} ] - {:?}",
                method.as_str(),
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or(""),
                elapsed,
                path.path(),
            );

            Ok(resp)
        }
    }
}