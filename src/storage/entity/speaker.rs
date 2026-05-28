use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "speakers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub languages_json: String,
    pub samples: i64,
    pub base_model: String,
    pub description: String,
    pub status: String,
    pub source: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
    pub deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
