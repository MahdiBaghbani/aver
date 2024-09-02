use argon2::{
    password_hash::{
        rand_core::OsRng
        , PasswordHasher, SaltString,
    },
    Argon2,
};
use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Form, Html};
use poem::{handler, IntoResponse, Result};
use tera::Context;

use aver_database::users::mutation::Mutation;

use crate::models::ApplicationState;
use crate::templates::TEMPLATES;

use super::models::CreateUserRequestData;

#[handler]
pub async fn create_ui() -> impl IntoResponse {
    let context: Context = Context::new();
    TEMPLATES
        .render("signup.html", &context)
        .map_err(InternalServerError)
        .map(Html)
        .unwrap()
        .into_response()
}

#[handler]
pub async fn create(
    state: Data<&ApplicationState>,
    Form(request): Form<CreateUserRequestData>,
) -> Result<impl IntoResponse> {
    let argon2: Argon2 = Argon2::default();
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let password_hash: String = argon2.hash_password(request.password.as_ref(), &salt)
        .unwrap()
        .to_string();

    Mutation::create_user(
        &state.database,
        request.firstname,
        request.lastname,
        request.username,
        password_hash,
    ).await.map_err(InternalServerError)?;

    Ok(StatusCode::CREATED)
}
