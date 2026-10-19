use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{env};
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize
}

pub struct TokenService;

impl TokenService {
    pub fn generate_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }
}
