use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateArtistViewObject {
    pub name: String,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub avatar: Option<String>,
    pub sex: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistQueryViewObject {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub genre: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ArtistDetailViewObject {
    pub id: uuid::Uuid,
    pub name: String,
    pub bio: String,
    pub genre: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub avatar_url: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}