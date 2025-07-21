use actix_web::{web, HttpResponse, Responder};

use super::super::{models, AppState};

// 获取歌手列表（支持按姓名和国籍筛选）
pub async fn get_artists(
    query: web::Query<models::ArtistQueryParams>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let artists = models::Artist::find_all(&state.config.db, &query)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch artists".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(artists),
        message: Some("Artists fetched successfully".to_string()),
    }))
}

// 根据ID获取歌手详情
pub async fn get_artist_by_id(
    artist_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let artist = models::Artist::find_by_id(&state.config.db, artist_id.into_inner())
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch artist".to_string()),
            })
        })?;

    Ok(match artist {
        Some(artist) => HttpResponse::Ok().json(models::ApiResponse {
            success: true,
            data: Some(artist),
            message: Some("Artist fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(models::ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Artist not found".to_string()),
        }),
    })
}

// 创建新歌手
pub async fn create_artist(
    data: web::Json<models::CreateArtistDataObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let artist = models::Artist::create(&state.config.db, &data)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to create artist".to_string()),
            })
        })?;

    Ok(HttpResponse::Created().json(models::ApiResponse {
        success: true,
        data: Some(artist),
        message: Some("Artist created successfully".to_string()),
    }))
}