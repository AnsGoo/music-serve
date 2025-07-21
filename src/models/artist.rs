use chrono::{ DateTime, Utc};
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, ActiveValue,DeriveEntityModel, QueryOrder, QuerySelect};
use sea_orm::entity::prelude::*;
use uuid::Uuid;


// 定义歌手表实体
#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "artist")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub sex: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<Date>,
    pub avatar: Option<String>,
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
    // 如果有相关表，可以在这里定义关联
}



impl sea_orm::ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::now_v7()),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            delete_flag: ActiveValue::Set(false),
            ..ActiveModelTrait::default()
        }
    }
}

// 专辑创建请求
#[derive(Debug, Deserialize)]
pub struct CreateArtistDataObject {
    pub name: String,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub avatar: Option<String>,
    pub created_by: String,
    pub sex: Option<String>,
}

// 专辑创建请求
#[derive(Debug, Deserialize)]
pub struct ArtistQueryParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub nationality: Option<String>,
    pub sex: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}


// 重命名为Artist以保持兼容性
pub type Artist = Model;


impl Artist {
    // 创建新歌手
    pub async fn create(db: &DatabaseConnection, data: &CreateArtistDataObject) -> Result<Self, DbErr> {
        let model = ActiveModel {
            name: ActiveValue::Set(data.name.clone()),
            nationality: ActiveValue::Set(data.nationality.clone()),
            birth_date: ActiveValue::Set(data.birth_date.as_ref().and_then(|d| Date::parse_from_str(d, "%Y-%m-%d").ok())),
            avatar: ActiveValue::Set(data.avatar.clone()),
            sex: ActiveValue::Set(data.sex.clone()), // 默认值为 None
            created_by: ActiveValue::Set(data.created_by.clone()),
            ..ActiveModelTrait::default()

        };
        model.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn find_all(db: &DatabaseConnection, params: &ArtistQueryParams) -> Result<Vec<Self>, DbErr> {
        let mut query = Entity::find().order_by_desc(Column::UpdatedAt).filter(Column::DeleteFlag.eq(false));

        if let Some(name) = &params.name {
            query = query.filter(Column::Name.contains(name));
        }
        
        if let Some(nationality) = &params.nationality {
            query = query.filter(Column::Nationality.eq(nationality));
        }
        if let Some(sex) = &params.sex {
            query = query.filter(Column::Sex.eq(sex));
        }
          let page = params.page.unwrap_or(1) as u64;
        let page_size = params.page_size.unwrap_or(20) as u64;
        let offset = ((page - 1) * page_size) as u64;
        query.limit(page_size).offset(offset).all(db).await
    }
    
}
