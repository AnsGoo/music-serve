use actix_web::{web, HttpResponse, Responder};

use super::super::{models, AppState};

// 获取歌手列表（支持按姓名和国籍筛选）
pub async fn get_singers(
    query: web::Query<models::SingerQueryParams>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let singers = models::Singer::find_all(&state.config.db, &query)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch singers".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(singers),
        message: Some("Singers fetched successfully".to_string()),
    }))
}

// 根据ID获取歌手详情
pub async fn get_singer_by_id(
    singer_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let singer = models::Singer::find_by_id(&state.config.db, singer_id.into_inner())
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch singer".to_string()),
            })
        })?;

    Ok(match singer {
        Some(singer) => HttpResponse::Ok().json(models::ApiResponse {
            success: true,
            data: Some(singer),
            message: Some("Singer fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(models::ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Singer not found".to_string()),
        }),
    })
}

// 创建新歌手
pub async fn create_singer(
    data: web::Json<models::CreateSingerRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let singer = models::Singer::create(&state.config.db, &data)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to create singer".to_string()),
            })
        })?;

    Ok(HttpResponse::Created().json(models::ApiResponse {
        success: true,
        data: Some(singer),
        message: Some("Singer created successfully".to_string()),
    }))
}