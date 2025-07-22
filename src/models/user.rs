use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::prelude::*;
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;

// 定义用户表实体
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: String,
    #[sea_orm(indexed)]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    #[sea_orm(indexed,default=false)]
    pub delete_flag: bool,
}

// 定义关联
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 如果有相关表，可以在这里定义关联
}

// 为Model实现ActiveModelTrait
impl ActiveModelBehavior for ActiveModel {
       fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::now_v7()),
            created_at: ActiveValue::Set(Utc::now().to_utc()),
            updated_at: ActiveValue::Set(Utc::now().to_utc()),
            delete_flag: ActiveValue::Set(false),
            ..ActiveModelTrait::default()
        }
    }
}

// 重命名为User以保持兼容性
pub type User = Model;

// 定义用户仓库trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbErr>;
    async fn create(&self, data: &CreateUserRequest) -> Result<User, DbErr>;
}

// 用户注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

// 用户登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password_hash: String,
}

// JWT响应
#[derive(Debug, Serialize)]
pub struct JwtResponse {
    pub token: String,
    pub expires_at: i64,
}

// API响应模型
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// 为ApiResponse实现Display trait
impl<T: Serialize> std::fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}",  self.message.clone().expect("No message").to_string())
    }
}

// SeaORM实现的用户仓库
pub struct SeaOrmUserRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            db,
        }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DbErr> {
        Entity::find()
            .filter(Column::Username.eq(username)).filter(Column::DeleteFlag.eq(false))
            .one(&*self.db)
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbErr> {
        Entity::find()
            .filter(Column::Email.eq(email)).filter(Column::DeleteFlag.eq(false))
            .one(&*self.db)
            .await
    }

    async fn create(&self, data: &CreateUserRequest) -> Result<User, DbErr> {
        let model = ActiveModel {
            username: ActiveValue::Set(data.username.clone()),
            password_hash: ActiveValue::Set(data.password_hash.clone()),
            role: ActiveValue::Set("user".to_string()),
            ..ActiveModel::new()
        };
        model.insert(&*self.db).await
    }
}

// 为User模型保留旧的数据访问方法（逐步迁移）
impl User {
    // 根据用户名查找用户
    pub async fn find_by_username(db: &DatabaseConnection, username: &str) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(Column::Username.eq(username)).filter(Column::DeleteFlag.eq(false))
            .one(db)
            .await
    }
    
    // 根据邮箱查找用户
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(Column::Email.eq(email)).filter(Column::DeleteFlag.eq(false))
            .one(db)
            .await
    }

    // 创建新用户
    pub async fn create(db: &DatabaseConnection, data: &CreateUserRequest) -> Result<Self, DbErr> {
        let model = ActiveModel {
            username: ActiveValue::Set(data.username.clone()),
            password_hash: ActiveValue::Set(data.password_hash.clone()),
            role: ActiveValue::Set("user".to_string()),
            ..ActiveModel::new()
        };
        model.insert(db).await
    }
}