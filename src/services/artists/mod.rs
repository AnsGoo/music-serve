pub mod model;
use crate::{models, AppState};
use crate::services::artists::model::ArtistDetailViewObject;
use actix_web::web;
use std::fmt;

#[derive(Debug)]
pub enum ArtistServiceError {
    DatabaseError(sea_orm::DbErr),
    ArtistNotFound,
}

impl fmt::Display for ArtistServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArtistServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            ArtistServiceError::ArtistNotFound => write!(f, "Artist not found"),
        }
    }
}

/// 获取歌手列表服务
pub async fn get_artists_service(
    query: models::ArtistQueryParams,
    state: &web::Data<AppState>,
) -> Result<Vec<model::ArtistDetailViewObject>, ArtistServiceError> {
    let artists = models::Artist::find_all(&state.config.db, &query)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    // 将DataObject转换为ViewObject
    let artist_view_objects = artists
        .into_iter()
        .map(|artist| model::ArtistDetailViewObject {
              id: artist.id,
              name: artist.name,
              bio: artist.nationality.clone().unwrap_or_default(),
              genre: "".to_string(),
              birth_date: Some(artist.birth_date.unwrap_or_default()),
              avatar_url: artist.avatar.clone().unwrap_or_default(),
              created_at: artist.created_at.naive_utc(),
              updated_at: artist.updated_at.naive_utc(),
          })
        .collect();

    Ok(artist_view_objects)
}

/// 根据ID获取歌手详情服务
pub async fn get_artist_by_id_service(
    artist_id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Option<ArtistDetailViewObject>, ArtistServiceError> {
    let artist = models::Artist::find_by_id(&state.config.db, artist_id)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    Ok(artist.map(|artist| ArtistDetailViewObject {
        id: artist.id,
        name: artist.name,
        bio: artist.nationality.clone().unwrap_or_default(),
        genre: "".to_string(),
        birth_date: Some(artist.birth_date.unwrap_or_default()),
        avatar_url: artist.avatar.clone().unwrap_or_default(),
        created_at: artist.created_at.naive_utc(),
        updated_at: artist.updated_at.naive_utc(),
    }))
}

/// 创建歌手服务
pub async fn create_artist_service(
    data: models::CreateArtistDataObject,
    state: &web::Data<AppState>,
) -> Result<ArtistDetailViewObject, ArtistServiceError> {
    // 创建歌手
    let artist = models::Artist::create(&state.config.db, &data)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    Ok(ArtistDetailViewObject {
        id: artist.id,
        name: artist.name,
        bio: artist.nationality.clone().unwrap_or_default(),
        genre: "".to_string(),
        birth_date: Some(artist.birth_date.unwrap_or_default()),
        avatar_url: artist.avatar.clone().unwrap_or_default(),
        created_at: artist.created_at.naive_utc(),
        updated_at: artist.updated_at.naive_utc(),
    })
}