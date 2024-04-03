use std::env;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::rngs::OsRng;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl Account {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: None,
            name,
            email,
            password: "".to_string(),
            access_token: "".to_string(),
            refresh_token: "".to_string(),
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn set_password(&mut self, password: String) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("create hash password error.")
            .to_string();

        self.password = password_hash;
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.password).expect("Falha ao parsear hash.");
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn set_tokens(&mut self) {
        let secret_key = env::var("AUTH_SECRET_KEY").expect("AUTH_SECRET_KEY must be set");
        let encoding_key = EncodingKey::from_secret(secret_key.as_ref());

        let expiration = Utc::now() + Duration::minutes(60);

        let my_claims = Claims {
            sub: self.id.clone().unwrap().to_string(),
            exp: expiration.timestamp() as usize,
        };

        let token = encode(&Header::default(), &my_claims, &encoding_key).unwrap();
        let refresh_token = Uuid::new_v4().to_string();

        self.access_token = token;
        self.refresh_token = refresh_token;
    }
}
