use actix_web::{web, App, HttpServer, middleware::Logger}; use middlewares::logger::RequestLogger;

mod config;
mod handlers;
mod models;
mod routers;
mod utils;
mod middlewares; // 确保这个模块声明是公开的，并且位于正确的位置


#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::AppConfig::new().await;
    let app_state = AppState { config: config.clone() };

    println!("Server running on @ http://localhost:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(RequestLogger)
            .configure(routers::configure)
    })
    .bind(format!("0.0.0.0:{}", config.port))?
    .run()
    .await
}
