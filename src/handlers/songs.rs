use actix_web::{web, HttpResponse, Responder};

use super::super::{models, AppState};

// 获取歌曲列表（支持按专辑、歌手、流派和音质筛选）
pub async fn get_songs(
    query: web::Query<models::SongQueryParams>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let songs = models::Song::find_all(&state.config.db, &query)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch songs".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(models::ApiResponse {
        success: true,
        data: Some(songs),
        message: Some("Songs fetched successfully".to_string()),
    }))
}

// 根据ID获取歌曲详情
pub async fn get_song_by_id(
    song_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let song = models::Song::find_by_id(&state.config.db, song_id.into_inner())
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch song".to_string()),
            })
        })?;

    Ok(match song {
        Some(song) => HttpResponse::Ok().json(models::ApiResponse {
            success: true,
            data: Some(song),
            message: Some("Song fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(models::ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Song not found".to_string()),
        }),
    })
}

// 创建新歌曲
pub async fn create_song(
    data: web::Json<models::CreateSongRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 验证专辑是否存在
    let album_exists = models::Album::find_by_id(&state.config.db, data.album_id)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to check album existence".to_string()),
            })
        })?;

    if album_exists.is_none() {
        return Ok(HttpResponse::BadRequest().json(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Album not found".to_string()),
            }));
    }

    // 验证歌手是否存在
    let singer_exists = models::Artist::find_by_id(&state.config.db, data.singer_id)
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

    // 创建歌曲
    let song = models::Song::create(&state.config.db, &data)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError(models::ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to create song".to_string()),
            })
        })?;

    Ok(HttpResponse::Created().json(models::ApiResponse {
        success: true,
        data: Some(song),
        message: Some("Song created successfully".to_string()),
    }))
}