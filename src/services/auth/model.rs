use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 登录请求视图对象
#[derive(Debug, Deserialize)]
pub struct LoginViewObject {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 注册请求视图对象
#[derive(Debug, Deserialize)]
pub struct RegisterViewObject {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 认证响应视图对象
#[derive(Debug, Serialize)]
pub struct AuthResponseViewObject {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: Option<String>,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间(秒)
    pub expires_in: i64,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseViewObject {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间(秒)
    pub expires_in: i64,
}