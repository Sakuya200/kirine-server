use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_type: String,
    pub title: String,
    pub speaker_id: Option<i64>,
    pub speaker_name: String,
    pub status: String,
    pub duration_seconds: i64,
    pub create_time: DateTime,
    pub modify_time: DateTime,
    pub finished_time: Option<DateTime>,
    pub device: String,
    pub deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
