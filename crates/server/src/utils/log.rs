use poem::http::{Method, Uri};
use poem::{Addr, Endpoint, IntoResponse, Request, Response};
use std::time::{Duration, Instant};
use tracing::{error, info};

pub async fn log<E: Endpoint>(next: E, req: Request) -> poem::Result<Response> {
    let method: Method = req.method().clone();
    let path: Uri = req.uri().clone();
    let client_ip: String = get_client_ip(&req).unwrap_or("Unknown IP".to_string());

    let start: Instant = Instant::now();
    let res = next.call(req).await;
    let elapsed: Duration = start.elapsed();

    match res {
        Ok(r) => {
            let resp = r.into_response();

            info!(
                "from {}: {} -> {} {} [ {:?} ] - {:?}",
                client_ip,
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
                "from {}: {} -> {} {} [ {:?} ] - {:?}",
                client_ip,
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

/// this function tries to get the client IP address from the headers. if the ip in header
/// not found, fallback to the remote address, which might be local proxy's ip address.
/// please note that when it comes with network policy, we need make sure the incoming
/// traffic comes from a trustworthy proxy instance.
pub fn get_client_ip(req: &Request) -> Option<String> {
    let headers: [&str; 3] = ["X-Real-IP", "X-Forwarded-For", "CF-Connecting-IP"];
    for &header in headers.iter() {
        if let Some(value) = req.headers().get(header) {
            if let Ok(mut ip_str) = value.to_str() {
                if header == "X-Forwarded-For" {
                    ip_str = ip_str.split(',').next().unwrap_or("");
                }
                return Some(ip_str.to_string());
            }
        }
    }

    // fallback to the connection's remote address, take care
    let client_ip = match req.remote_addr().0 {
        Addr::SocketAddr(addr) => Some(addr.ip().to_string()),
        Addr::Custom(..) => Some("127.0.0.1".to_string()),
    };

    client_ip
}
