use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Json};
use poem::{handler, IntoResponse, Result};

use aver_database::sea_orm::DatabaseConnection;
use aver_database::users::mutation::Mutation;

use super::models::CreateUserRequest;
use crate::models::ApplicationState;

#[handler]
pub async fn create(
    state: Data<&ApplicationState>,
    Json(data): Json<CreateUserRequest>,
) -> Result<impl IntoResponse> {
    let database: &DatabaseConnection = &state.database;

    Mutation::create_user(
        database,
        data.first_name,
        data.last_name,
        data.username,
        data.password,
    ).await.map_err(InternalServerError)?;

    Ok(StatusCode::CREATED)
}
