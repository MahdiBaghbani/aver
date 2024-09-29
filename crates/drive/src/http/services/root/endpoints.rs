use poem::error::InternalServerError;
use poem::session::Session;
use poem::web::Html;
use poem::{handler, IntoResponse};
use tera::Context;


use crate::templates::TEMPLATES;

#[handler]
pub async fn index(session: &Session) -> impl IntoResponse {
    let mut context: Context = Context::new();
    
    match session.get::<String>("username") {
        Some(username) => {
            context.insert("username", &username);
            context.insert("is_authenticated", &true);
        }
        None => {
            context.insert("is_authenticated", &false);
        }
    }

    TEMPLATES
        .render("index.html", &context)
        .map_err(InternalServerError)
        .map(Html)
        .unwrap()
        .into_response()
}
