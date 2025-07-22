pub mod model;
use self::model::*;
use crate::models;
use std::fmt;
use std::sync::Arc;
use crate::models::album::AlbumRepository;
use crate::models::artist::ArtistRepository;

#[derive(Debug)]
pub enum AlbumServiceError {
    DatabaseError(sea_orm::DbErr),
    ArtistNotFound,
    AlbumNotFound,
}

impl fmt::Display for AlbumServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlbumServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AlbumServiceError::ArtistNotFound => write!(f, "Artist not found"),
            AlbumServiceError::AlbumNotFound => write!(f, "Album not found"),
        }
    }
}

/// 获取专辑列表服务
pub async fn get_albums_service(
    query: models::AlbumQueryParams,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>
) -> Result<Vec<AlbumDetailViewObject>, AlbumServiceError> {
    let albums = album_repo.find_all(&query)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    // 将DataObject转换为ViewObject
    let album_view_objects = albums
        .into_iter()
        .map(|album| AlbumDetailViewObject {
            id: album.id,
            name: album.name,
            artist_id: album.artist_id,
            cover_image: album.cover_image.unwrap_or_default(),
            release_date: album.release_date,
            description: album.description,
            created_at: album.created_at,
            updated_at: album.updated_at,
        })
        .collect();

    Ok(album_view_objects)
}

/// 根据ID获取专辑详情服务
pub async fn get_album_by_id_service(
    album_id: uuid::Uuid,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>
) -> Result<Option<AlbumDetailViewObject>, AlbumServiceError> {
    let album = album_repo.find_by_id(album_id)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    Ok(album.map(|album| AlbumDetailViewObject {
        id: album.id,
        name: album.name,
        artist_id: album.artist_id,
        cover_image: album.cover_image.unwrap_or_default(),
        release_date: album.release_date,
        description: album.description,
        created_at: album.created_at,
        updated_at: album.updated_at,
    }))
}

/// 创建专辑服务
pub async fn create_album_service(
    data: models::CreateAlbumRequest,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>
) -> Result<AlbumDetailViewObject, AlbumServiceError> {
    // 验证歌手是否存在
    let artist_exists = artist_repo.find_by_id(data.artist_id)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    if artist_exists.is_none() {
        return Err(AlbumServiceError::ArtistNotFound);
    }

    // 创建专辑
    let album = album_repo.create(&data)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    Ok(AlbumDetailViewObject {
        id: album.id,
        name: album.name,
        artist_id: album.artist_id,
        cover_image: album.cover_image.unwrap_or_default(),
        release_date: album.release_date,
        description: album.description,
        created_at: album.created_at,
        updated_at: album.updated_at,
    })
}