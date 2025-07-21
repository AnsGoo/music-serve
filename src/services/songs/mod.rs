pub mod model;
use crate::{models, AppState};
use sea_orm::DbErr; 
use uuid::Uuid;

// 获取歌曲列表服务
pub async fn get_songs_service(
    query: models::SongQueryParams,
    state: &AppState
) -> Result<Vec<models::Song>, DbErr> {
    models::Song::find_all(&state.config.db, &query).await
}

// 根据ID获取歌曲详情服务
pub async fn get_song_by_id_service(
    song_id: Uuid,
    state: &AppState
) -> Result<Option<models::Song>, DbErr> {
    models::Song::find_by_id(&state.config.db, song_id).await
}

// 创建歌曲服务
pub async fn create_song_service(
    data: models::CreateSongRequest,
    state: &AppState
) -> Result<models::Song, DbErr> {
    // 验证专辑是否存在
    let album_exists = models::Album::find_by_id(&state.config.db, data.album_id).await?;
    if album_exists.is_none() {
        return Err(DbErr::Custom("Album not found".to_string()));
    }

    // 验证歌手是否存在
    let artist_exists = models::Artist::find_by_id(&state.config.db, data.artist_id).await?;
    if artist_exists.is_none() {
        return Err(DbErr::Custom("Artist not found".to_string()));
    }

    models::Song::create(&state.config.db, &data).await
}