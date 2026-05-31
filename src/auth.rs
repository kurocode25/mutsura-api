use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, string::String};
use uuid::Uuid;

// JWTの構造体
#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// JWTを使って認証する際の処理
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // シークレットキーの準備
        dotenv().ok();
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        // トークンの取得と認証
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// リフレッシュトークンの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    pub token: String,
    pub exp: DateTime,
    pub revoked: bool,
    pub user_id: String,
}

// JWTの発行処理
pub fn issue_jwt(sub: &str) -> Result<String, AuthError> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let iat = Utc::now().timestamp() as usize;
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        iat,
        exp,
    };

    encode(&header, &claims, &encoding_key).map_err(|_| AuthError::TokenCreation)
}

// JWTのデコード処理
pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}

// リフレッシュトークンの認証処理
pub fn check_refresh_token(token: &RefreshToken) -> bool {
    // RefreshToken.revokedの検証
    // 有効期限の検証。発行から7日以内なら有効
    // リフレッシュトークンが有効の可否を返す。
    let now = DateTime::now();
    !token.revoked && token.exp >= now
}

// リフレッシュトークンの発行処理
pub fn issue_refresh_token(id: &str) -> RefreshToken {
    let token = Uuid::new_v4().to_string();
    let exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp");

    RefreshToken {
        token,
        exp: exp.into(),
        revoked: false,
        user_id: id.to_string(),
    }
}

// 認証エラー
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    BadRequest,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
