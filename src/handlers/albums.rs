use actix_web::{web, HttpResponse, Responder};
// 移除SQLx引用

use super::super::{models, AppState};

// 获取专辑列表（支持按歌手、名称和发行日期筛选）
pub async fn get_albums(
    query: web::Query<models::AlbumQueryParams>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let albums = models::Album::find_all(&state.config.db, &query)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch albums".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(albums),
        message: Some("Albums fetched successfully".to_string()),
    }))
}

// 根据ID获取专辑详情
pub async fn get_album_by_id(
    album_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let album = models::Album::find_by_id(&state.config.db, album_id.into_inner())
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch album".to_string()),
            })
        })?;

    Ok(match album {
        Some(album) => HttpResponse::Ok().json(models::ApiResponse {
            success: true,
            data: Some(album),
            message: Some("Album fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(models::ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Album not found".to_string()),
        }),
    })
}

// 创建新专辑
pub async fn create_album(
    data: web::Json<models::CreateAlbumRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 验证歌手是否存在
    let singer_exists = models::Singer::find_by_id(&state.config.db, data.singer_id)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to check singer existence".to_string()),
            })
        })?;

    if singer_exists.is_none() {
        return Ok(HttpResponse::BadRequest().json(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Singer not found".to_string()),
            }));
    }

    // 创建专辑
    let album = models::Album::create(&state.config.db, &data)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to create album".to_string()),
            })
        })?;

    Ok(HttpResponse::Created().json(models::ApiResponse {
        success: true,
        data: Some(album),
        message: Some("Album created successfully".to_string()),
    }))
}