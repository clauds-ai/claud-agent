use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum TokenError {
    JwtError(jsonwebtoken::errors::Error),
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenError::JwtError(e) => write!(f, "JWT error: {}", e),
        }
    }
}

impl std::error::Error for TokenError {}

impl From<jsonwebtoken::errors::Error> for TokenError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        TokenError::JwtError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub struct AuthService {
    pub secret: Vec<u8>,
}

impl AuthService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.as_bytes().to_vec(),
        }
    }

    pub fn create_token(
        &self,
        subject: String,
        expires_in: chrono::Duration,
    ) -> Result<String, TokenError> {
        let claims = Claims {
            sub: subject,
            exp: (chrono::Utc::now() + expires_in).timestamp(),
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )?;
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, TokenError> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(decoded.claims)
    }
}
