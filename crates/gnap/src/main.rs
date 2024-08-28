use mimalloc::MiMalloc;
use poem::{listener::TcpListener, Server};
use tracing::{debug, info};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

mod http;
mod settings;
mod errors;
mod models;

use crate::http::application;
use crate::settings::settings;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // loads the .env file from the current directory or parents.
    dotenvy::dotenv_override().ok();

    // load settings from toml file.
    settings::init();

    // setup logging.
    let filter: EnvFilter = EnvFilter::builder()
        .with_default_directive(LevelFilter::ERROR.into())
        .parse_lossy(settings().log.level.clone());
    let filtered_layer = fmt::layer().with_level(true).with_filter(filter);
    tracing_subscriber::registry().with(filtered_layer).init();

    info!("⚙️ Settings have been loaded.");
    debug!("{:#?}", settings());

    let tcp_bind: String = settings().server.get_tcp_bind();
    let tcp_listener: TcpListener<String> = TcpListener::bind(tcp_bind);

    Server::new(tcp_listener).run(application()).await
}
