use tracing::{debug, info};

use aver_database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use aver_database_migration::{Migrator, MigratorTrait};

pub async fn setup_database(uri: String) -> DatabaseConnection {
    // database connection options.
    let mut options: ConnectOptions = ConnectOptions::new(uri);
    options.sqlx_logging(false);

    debug!("⚙️ Performing database connection.");
    let connection: DatabaseConnection = Database::connect(options).await.unwrap();
    debug!("⚙️ Database connection established.");

    debug!("⚙️ Applying all pending database migrations");
    Migrator::up(&connection, None).await.unwrap();
    info!("⚙️ Database Initialization is complete.");

    connection
}
