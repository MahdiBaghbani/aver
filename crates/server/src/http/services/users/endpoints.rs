use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Json};
use poem::{handler, IntoResponse, Result};

use aver_database::users::mutation::Mutation;

use super::models::CreateUserRequestData;
use crate::models::ApplicationState;

#[handler]
pub async fn create(
    state: Data<&ApplicationState>,
    Json(data): Json<CreateUserRequestData>,
) -> Result<impl IntoResponse> {
    Mutation::create_user(
        &state.database,
        data.first_name,
        data.last_name,
        data.username,
        data.password,
    ).await.map_err(InternalServerError)?;

    Ok(StatusCode::CREATED)
}
