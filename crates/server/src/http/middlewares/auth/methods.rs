use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use poem::http::header::AUTHORIZATION;
use poem::{Endpoint, Middleware, Request};
use poem_grants::authorities::AttachAuthorities;
use poem_grants::error::AccessError::UnauthorizedRequest;

use crate::http::middlewares::auth::constants::{JWT_EXPIRATION_HOURS, SECRET};
use crate::http::middlewares::auth::models::{Claims, JwtMiddleware, JwtMiddlewareImpl};

impl<E: Endpoint> Middleware<E> for JwtMiddleware {
    type Output = JwtMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        JwtMiddlewareImpl { ep }
    }
}

impl<E: Endpoint> Endpoint for JwtMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        // Just example. You have to make sure your middleware is correct
        if let Some(value) = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..])
        {
            // Decode JWT token
            let claims = decode_jwt(value)?;
            // Attache permissions to request for `poem-grants`
            req.attach(claims.permissions);
        }

        // call the next endpoint.
        self.ep.call(req).await
    }
}

impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::try_hours(JWT_EXPIRATION_HOURS).unwrap()).timestamp(),
        }
    }
}

// Create a json web token (JWT)
pub fn create_jwt(claims: Claims) -> poem::Result<String> {
    let encoding_key: EncodingKey = EncodingKey::from_secret(SECRET.as_bytes());

    // Just example, here should be the correct way to handle the error
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| UnauthorizedRequest.into())
}

// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> poem::Result<Claims> {
    let decoding_key: DecodingKey = DecodingKey::from_secret(SECRET.as_bytes());

    // Just example, here should be the correct way to handle the error
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| UnauthorizedRequest.into())
}
