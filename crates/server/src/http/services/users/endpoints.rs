use argon2::{
    password_hash::{
        rand_core::OsRng
        , PasswordHasher, SaltString,
    },
    Argon2,
};
use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Form, Html, Json};
use poem::{handler, IntoResponse, Result};
use tera::Context;

use aver_database::users::mutation::Mutation;
use aver_database_entity::users;

use crate::models::ApplicationState;
use crate::templates::TEMPLATES;

use super::models::{RegisterUserRequestData, RegisterUserResponseData};

#[handler]
pub async fn register_ui() -> impl IntoResponse {
    let context: Context = Context::new();
    TEMPLATES
        .render("signup.html", &context)
        .map_err(InternalServerError)
        .map(Html)
        .unwrap()
        .into_response()
}

#[handler]
pub async fn register_with_form(
    state: Data<&ApplicationState>,
    Form(request): Form<RegisterUserRequestData>,
) -> Result<impl IntoResponse> {
    register(state, request).await
}

#[handler]
pub async fn register_with_json(
    state: Data<&ApplicationState>,
    Json(request): Json<RegisterUserRequestData>,
) -> Result<impl IntoResponse> {
    register(state, request).await
}

pub async fn register(
    state: Data<&ApplicationState>,
    data: RegisterUserRequestData,
) -> Result<impl IntoResponse> {
    let argon2: Argon2 = Argon2::default();
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let password_hash: String = argon2.hash_password(
        data.password.as_ref(),
        &salt,
    ).unwrap().to_string();

    let user: users::Model = Mutation::create_user(
        &state.database,
        data.firstname,
        data.lastname,
        data.username,
        password_hash,
    ).await.map_err(InternalServerError)?;

    Ok(
        (
            StatusCode::CREATED,
            Json(
                RegisterUserResponseData {
                    id: user.id,
                    firstname: user.first_name,
                    lastname: user.last_name,
                    username: user.username,
                }
            )
        )
    )
}
