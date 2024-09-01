use aver_database::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub database: DatabaseConnection,
}
