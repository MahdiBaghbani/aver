use aver_database::sea_orm::DatabaseConnection;
use mimalloc::MiMalloc;
use poem::{listener::TcpListener, Server};
use tracing::{info, trace};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

mod settings;
mod http;
mod models;
mod utils;
mod database;
mod templates;

use self::database::setup_database;
use self::http::application;
use self::models::ApplicationState;
use self::settings::settings;

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
    trace!("{:#?}", settings());

    info!("⚙️ Initialize Database.");
    let database: DatabaseConnection = setup_database(
        settings().database.get_uri()
    ).await;

    let state: ApplicationState = ApplicationState {
        database
    };

    let tcp_bind: String = settings().server.get_tcp_bind();
    let tcp_listener: TcpListener<String> = TcpListener::bind(tcp_bind);

    info!("⚙️ Starting Server.");
    Server::new(tcp_listener).run(application(state).await).await
}
