use sea_orm::entity::prelude::*;
use chrono::Utc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelatedEntity, DeriveRelation, EnumIter};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_id: Uuid,
    pub refresh_token_id: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    pub expires_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

impl ActiveModelBehavior for ActiveModel {}