use actix_web::{web, Error, HttpResponse};
use actix_identity::Identity;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use bcrypt::verify;
use crate::config::Config;
use crate::services::user_service::{find_by_username, verify_password};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    let user = find_by_username(&auth_data.username).await?;
    if let Some(user) = user {
        if verify_password(&user, &auth_data.password).await? {
            let claims = Claims {
                sub: user.username.clone(),
                exp: 10000000000,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            )?;
            id.remember(token);
            Ok(HttpResponse::Ok().json(user))
        } else {
            Ok(HttpResponse::Unauthorized().finish())
        }
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

pub async fn authenticate_jwt(
    id: Identity,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    if let Some(token) = id.identity() {
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::Unauthorized().finish()),
        }
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
