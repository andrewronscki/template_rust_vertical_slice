use std::env;

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::accounts::domain::account::Claims;

pub async fn verify_jwt(mut req: Request<Body>, next: Next) -> Result<Response<Body>, StatusCode> {
    let token = match req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            env::var("AUTH_SECRET_KEY")
                .expect("AUTH_SECRET_KEY must be set")
                .as_ref(),
        ),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded) => {
            let claims = decoded.claims;
            req.extensions_mut().insert(claims.clone());
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
