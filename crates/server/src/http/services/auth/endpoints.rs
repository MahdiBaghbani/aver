use crate::http::services::auth::models::LoginRequestData;
use crate::templates::TEMPLATES;
use poem::error::InternalServerError;
use poem::http::{header, StatusCode};
use poem::session::Session;
use poem::web::{Form, Html};
use poem::{handler, IntoResponse, Response};
use tera::Context;

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
pub async fn login_form(Form(params): Form<LoginRequestData>, session: &Session) -> impl IntoResponse {
    if params.username == "test" && params.password == "123456" {
        session.set("username", params.username);
        Response::builder()
            .status(StatusCode::FOUND)
            .header(header::LOCATION, "/")
            .finish()
    } else {
        Html(
            r#"
    <!DOCTYPE html>
    <html>
    <head><meta charset="UTF-8"><title>Example Session Auth</title></head>
    <body>
    no such user
    </body>
    </html>
    "#,
        )
            .into_response()
    }
}
