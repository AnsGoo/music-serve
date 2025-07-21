use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelBehavior,ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, DatabaseConnection, DeriveRelation, EnumIter, DeriveEntityModel, prelude::*};
use uuid::Uuid;

// 定义专辑表实体
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "album")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub singer_id: Uuid,
    #[sea_orm(column_type = "Text", indexed, nullable)]
    pub name: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub genre: Option<String>,
    #[sea_orm(column_type = "DateTime", indexed)]
    pub release_date: NaiveDate,
    #[sea_orm(indexed)]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: String,
    #[sea_orm(indexed,default=false)]
    pub delete_flag: bool,
}

// 定义关联
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 关联歌手表
    #[sea_orm(belongs_to = "super::artist::Entity", from = "Column::SingerId", to = "super::artist::Column::Id")]
    Singer,
}

// 为ActiveModel实现ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::now_v7()),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

// 重命名为Album以保持兼容性
pub type Album = Model;

// 专辑创建请求
#[derive(Debug, Deserialize)]
pub struct CreateAlbumRequest {
    pub singer_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub genre: Option<String>,
    pub release_date: NaiveDate,
}

// 专辑查询参数
#[derive(Debug, Deserialize)]
pub struct AlbumQueryParams {
    pub singer_id: Option<Uuid>,
    pub name: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// 为Album模型添加数据访问方法
impl Album {
    // 创建新专辑
    pub async fn create(db: &DatabaseConnection, request: &CreateAlbumRequest) -> Result<Self, DbErr> {
        let album = ActiveModel {
            singer_id: ActiveValue::Set(request.singer_id),
            name: ActiveValue::Set(request.name.clone()),
            description: ActiveValue::Set(request.description.clone()),
            cover_image: ActiveValue::Set(request.cover_image.clone()),
            genre: ActiveValue::Set(request.genre.clone()),
            release_date: ActiveValue::Set(request.release_date),
            ..ActiveModel::new()
        };

        album.insert(db).await
    }

    // 根据ID获取专辑
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(Column::Id.eq(id)).filter(Column::DeleteFlag.eq(false))
            .one(db)
            .await
    }

    // 获取所有专辑（支持筛选和分页）
    pub async fn find_all(db: &DatabaseConnection, params: &AlbumQueryParams) -> Result<Vec<Self>, DbErr> {
        let mut query = Entity::find().order_by_desc(Column::ReleaseDate).filter(Column::DeleteFlag.eq(false));

        // 添加筛选条件
        if let Some(singer_id) = &params.singer_id {
            query = query.filter(Column::SingerId.eq(*singer_id));
        }

        if let Some(name) = &params.name {
            query = query.filter(Column::Name.like(format!("%{}%", name)));
        }

        if let Some(release_date) = &params.release_date {
            query = query.filter(Column::ReleaseDate.eq(*release_date));
        }

        // 处理分页
        let page = params.page.unwrap_or(1) as u64;
        let page_size = params.page_size.unwrap_or(20) as u64;
        let offset = ((page - 1) * page_size) as u64;

        query
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await
    }
}