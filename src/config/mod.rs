use dotenv::dotenv;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};

#[derive(Clone)]
pub struct AppConfig {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
    pub port: String,
}

impl AppConfig {
    pub async fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        println!("Using port: {}", port);
        
        
        
        // 初始化日志系统
        if let Err(e) = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init() {
            eprintln!("Failed to initialize tracing subscriber: {}", e);
        }


        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(true) // Disable SQLx log
            .sqlx_logging_level(log::LevelFilter::Debug); // Or set SQLx log level   

        // 创建数据库连接
  let db = Database::connect(opt).await.expect("Failed to connect to database");

        AppConfig {
            db,
            jwt_secret,
            port,
        }
    }
}