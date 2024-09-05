use argon2::{Argon2, PasswordHash, PasswordVerifier};
use poem::error::InternalServerError;
use poem::http::{header, StatusCode};
use poem::session::Session;
use poem::web::{Data, Form, Html, Json};
use poem::{handler, IntoResponse, Response, Result};
use tera::Context;

use aver_database::users::query::Query;
use aver_database_entity::users;

use crate::models::ApplicationState;
use crate::templates::TEMPLATES;

use super::models::{LoginMode, LoginRequestData};

#[handler]
pub async fn login_ui() -> impl IntoResponse {
    let context: Context = Context::new();
    TEMPLATES
        .render("login.html", &context)
        .map_err(InternalServerError)
        .map(Html)
        .unwrap()
        .into_response()
}

#[handler]
pub async fn login_with_form(
    state: Data<&ApplicationState>,
    session: &Session,
    Form(request): Form<LoginRequestData>,
) -> Result<impl IntoResponse> {
    login(&state, session, &request, LoginMode::Cookie).await
}

#[handler]
pub async fn login_with_json(
    state: Data<&ApplicationState>,
    session: &Session,
    Json(request): Json<LoginRequestData>,
) -> Result<impl IntoResponse> {
    login(&state, session, &request, LoginMode::Token).await
}


pub async fn login(
    state: &Data<&ApplicationState>,
    session: &Session,
    request: &LoginRequestData,
    mode: LoginMode,
) -> Result<impl IntoResponse> {
    if request.username.is_empty() | request.password.is_empty() {
        return Ok(
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .finish()
        );
    }

    let result: Option<users::Model> = Query::find_user_by_username(
        &state.database,
        &request.username,
    ).await.map_err(InternalServerError)?;

    let response: Response = match result {
        Some(user) => {
            let result = PasswordHash::new(&user.password);
            match result {
                Ok(parsed_hash) => {
                    let valid_password: bool = Argon2::default().verify_password(
                        request.password.as_ref(),
                        &parsed_hash,
                    ).is_ok();
                    if valid_password {
                        match mode {
                            LoginMode::Cookie => {
                                session.set("username", &request.username);
                            }
                            LoginMode::Token => {}
                        }

                        Response::builder()
                            .status(StatusCode::OK)
                            .finish()
                    } else {
                        Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .finish()
                    }
                }
                Err(_) => {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
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
pub async fn logout(session: &Session) -> impl IntoResponse {
    session.purge();
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/")
        .finish()
}
