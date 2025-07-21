// 导出所有模型模块
// 导出实体和相关类型
pub mod user;
pub use user::{User, RegisterRequest, LoginRequest, JwtResponse, ApiResponse};

pub mod artist;
pub use artist::{Artist, ArtistQueryParams,CreateArtistDataObject};

pub mod album;
pub use album::{Album, CreateAlbumRequest, AlbumQueryParams};

pub mod song;
pub use song::{Song, CreateSongRequest, SongQueryParams};