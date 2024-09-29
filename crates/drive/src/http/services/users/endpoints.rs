use argon2::{
    password_hash::{
        rand_core::OsRng
        , PasswordHasher, SaltString,
    },
    Argon2,
};
use poem::error::InternalServerError;
use poem::http::{header::CONTENT_TYPE, StatusCode};
use poem::session::Session;
use poem::web::{Data, Form, Html, Json};
use poem::{handler, IntoResponse, Response, Result};
use tera::Context;

use aver_database::users::mutation::Mutation;
use aver_database::users::query::Query;
use aver_database_entity::users;

use crate::models::ApplicationState;
use crate::templates::TEMPLATES;

use super::models::{RegisterUserRequestData, UserInfoResponseData};

#[handler]
pub async fn me(
    state: Data<&ApplicationState>,
    session: &Session,
) -> Result<impl IntoResponse> {
    let response: Response = match session.get::<String>("username") {
        Some(username) => {
            let result: Option<users::Model> = Query::find_user_by_username(
                &state.database,
                &username,
            ).await.map_err(InternalServerError)?;

            match result {
                Some(user) => {
                    let user_info = UserInfoResponseData {
                        id: user.id,
                        firstname: user.first_name,
                        lastname: user.last_name,
                        username: user.username,
                    };

                    let body: Vec<u8> = serde_json::to_vec(&user_info).map_err(InternalServerError)?;

                    Response::builder()
                        .status(StatusCode::OK)
                        .header(CONTENT_TYPE, "application/json; charset=utf-8")
                        .body(body)
                }
                None => {
                    Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .finish()
                }
            }
        }
        None => {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .finish()
        }
    };

    Ok(response)
}

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

    let result: users::Model = Mutation::create_user(
        &state.database,
        data.firstname,
        data.lastname,
        data.username,
        password_hash,
    ).await.map_err(InternalServerError)?;

    let user_info: UserInfoResponseData = UserInfoResponseData {
        id: result.id,
        firstname: result.first_name,
        lastname: result.last_name,
        username: result.username,
    };

    let body: Vec<u8> = serde_json::to_vec(&user_info).map_err(InternalServerError)?;

    let response: Response = Response::builder()
        .status(StatusCode::CREATED)
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .body(body);

    Ok(response)
}
