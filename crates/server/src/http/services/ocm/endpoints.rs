use chrono::{Duration, Utc};
use poem::error::InternalServerError;
use poem::http::header::CONTENT_TYPE;
use poem::http::HeaderMap;
use poem::session::Session;
use poem::web::{Data, Html};
use poem::{
    handler,
    http::StatusCode,
    web::Json,
    web::Redirect,
    IntoResponse,
    Response,
    Result,
};
use tera::Context;
use uuid::Uuid;

use aver_database::ocm::mutation::Mutation;
use aver_database::users::query::Query;
use aver_database_entity::ocm_invite_tokens;
use aver_database_entity::users;

use crate::http::EndpointMode;
use crate::models::ApplicationState;
use crate::settings::settings;
use crate::templates::TEMPLATES;

use super::models::{
    CreateInviteTokenResponseData,
    DiscoveryData,
    InviteAcceptedRequestData,
    InviteAcceptedResponseData,
};

#[handler]
pub async fn discovery() -> impl IntoResponse {
    let discovery_data: DiscoveryData = DiscoveryData::new(
        settings().ocm.provider.clone()
    );

    let mut headers: HeaderMap = HeaderMap::try_with_capacity(1).unwrap();
    headers.insert(CONTENT_TYPE, "application/hal+json".parse().unwrap());

    (StatusCode::OK, headers, Json(discovery_data))
}

#[handler]
pub async fn legacy_discovery() -> Redirect {
    let uri: String = format!("{}/.well-known/ocm", settings().server.get_url());
    Redirect::moved_permanent(uri)
}

#[handler]
pub async fn mfa_capable() -> impl IntoResponse {
    if settings().ocm.provider.capabilities.mfa_capable {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[handler]
pub async fn create_invite_token_ui() -> impl IntoResponse {
    let mut context: Context = Context::new();
    context.insert("token", "");
    TEMPLATES
        .render("create-invite-token.html", &context)
        .map_err(InternalServerError)
        .map(Html)
        .unwrap()
        .into_response()
}

#[handler]
pub async fn create_invite_token_return_html(
    state: Data<&ApplicationState>,
    session: &Session,
) -> Result<impl IntoResponse> {
    create_invite_token(&state, session, EndpointMode::Ssr).await
}

#[handler]
pub async fn create_invite_token_return_json(
    state: Data<&ApplicationState>,
    session: &Session,
) -> Result<impl IntoResponse> {
    create_invite_token(&state, session, EndpointMode::Api).await
}


pub async fn create_invite_token(
    state: &Data<&ApplicationState>,
    session: &Session,
    mode: EndpointMode,
) -> Result<impl IntoResponse> {
    let response: Response = match session.get::<String>("username") {
        Some(username) => {
            let result: Option<users::Model> = Query::find_user_by_username(
                &state.database,
                &username,
            ).await.map_err(InternalServerError)?;

            match result {
                Some(user) => {
                    let token: String = Uuid::new_v4().to_string();

                    let expiration_time: i64 = (
                        Utc::now() + Duration::try_minutes(1).unwrap()
                    ).timestamp();

                    let model: ocm_invite_tokens::Model = Mutation::create_token(
                        &state.database,
                        user.id,
                        token,
                        expiration_time,
                    ).await.map_err(InternalServerError)?;

                    match mode {
                        EndpointMode::Api => {
                            let token = CreateInviteTokenResponseData {
                                token: model.token,
                                expiration_time: model.expiration_time,
                            };

                            let body: Vec<u8> = serde_json::to_vec(&token).map_err(InternalServerError)?;

                            Response::builder()
                                .status(StatusCode::OK)
                                .header(CONTENT_TYPE, "application/json; charset=utf-8")
                                .body(body)
                        }
                        EndpointMode::Ssr => {
                            let mut context: Context = Context::new();
                            context.insert("token", &model.token);
                            TEMPLATES
                                .render("create-invite-token.html", &context)
                                .map_err(InternalServerError)
                                .map(Html)?
                                .into_response()
                        }
                    }
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
pub async fn invite_accepted(Json(_request): Json<InviteAcceptedRequestData>) -> impl IntoResponse {
    let resp = InviteAcceptedResponseData {
        user_id: "1".to_string(),
        email: "mahdi-baghbani@azadehafzar.io".to_string(),
        name: "Mahdi Baghbani".to_string(),
    };

    (StatusCode::OK, Json(resp))
}
