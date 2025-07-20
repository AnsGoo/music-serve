use actix_web::{web, HttpResponse, Responder};



use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use chrono::{Utc, Duration};  use serde::{Serialize, Deserialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Claims {
      pub sub: String,
      exp: i64,
  }
use uuid::Uuid;

use super::super::{models, AppState};

// 用户注册
pub async fn register(
    data: web::Json<models::RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 检查邮箱是否已存在
    let existing_user = models::User::find_by_username(&state.config.db, &data.username)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to check existing user".to_string()),
            })
        })?;

    if existing_user.is_some() {
        return Ok(HttpResponse::Conflict().json(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("user already registered".to_string()),
            }));
    }

    // 密码哈希
    let password_hash = hash(&data.password, DEFAULT_COST)
        .map_err(|e| {
            log::error!("Password hash error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to hash password".to_string()),
            })
        })?;

    // 创建新用户
    let user = models::User::create(&state.config.db, &data, &password_hash)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to create user".to_string()),
            })
        })?;

    // 生成JWT
    let expires_at = (Utc::now() + Duration::hours(24)).timestamp();
    let token = generate_jwt(user.id, &state.config.jwt_secret, expires_at)
        .map_err(|e| {
            log::error!("JWT generation error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to generate token".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(models::JwtResponse { token, expires_at }),
        message: Some("User registered successfully".to_string()),
    }))
}

// 用户登录
pub async fn login(
    data: web::Json<models::LoginRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 查询用户
    let user = models::User::find_by_username(&state.config.db, &data.username) 
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to query user".to_string()),
            })
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::Unauthorized().json(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Invalid username or password".to_string()),
            }));
        }
    };

    // 验证密码
    let password_valid = verify(&data.password, &user.password_hash)
        .map_err(|e| {
            log::error!("Password verify error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to verify password".to_string()),
            })
        })?;

    if !password_valid {
        return Ok(HttpResponse::Unauthorized().json(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Invalid username or password".to_string()),
            }));
    }

    // 生成JWT
    let expires_at = (Utc::now() + Duration::hours(24)).timestamp();
    let token = generate_jwt(user.id, &state.config.jwt_secret, expires_at)
        .map_err(|e| {
            log::error!("JWT generation error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to generate token".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(models::JwtResponse { token, expires_at }),
        message: Some("Login successful".to_string()),
    }))
}

// 生成JWT令牌
fn generate_jwt(user_id: Uuid, secret: &str, expires_at: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expires_at,
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
