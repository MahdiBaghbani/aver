use poem::http::{Method, Uri};
use poem::{Endpoint, IntoResponse, Request, Response};
use std::time::{Duration, Instant};
use tracing::{error, info};

pub async fn log<E: Endpoint>(next: E, req: Request) -> poem::Result<Response> {
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