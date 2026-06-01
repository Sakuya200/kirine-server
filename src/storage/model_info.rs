use std::io;

use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
};

use crate::data_model::model_info::types::ModelDownloadState;
use crate::storage::LocalStorage;
use crate::storage::entity::model_info;
use crate::utils::current_native_time;

#[async_trait]
pub trait ModelInfoStorage {
    async fn list_model_info_entities(&self) -> Result<Vec<model_info::Model>>;
    async fn get_model_info_entity(&self, model_id: i64) -> Result<model_info::Model>;
    async fn update_model_download_state(
        &self,
        model_id: i64,
        state: ModelDownloadState,
    ) -> Result<model_info::Model>;
}

#[async_trait]
impl ModelInfoStorage for LocalStorage {
    async fn list_model_info_entities(&self) -> Result<Vec<model_info::Model>> {
        model_info::Entity::find()
            .filter(model_info::Column::Deleted.eq(0))
            .order_by_asc(model_info::Column::Id)
            .all(&self.db_conn)
            .await
            .map_err(Into::into)
    }

    async fn get_model_info_entity(&self, model_id: i64) -> Result<model_info::Model> {
        model_info::Entity::find_by_id(model_id)
            .filter(model_info::Column::Deleted.eq(0))
            .one(&self.db_conn)
            .await?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到目标模型").into())
    }

    async fn update_model_download_state(
        &self,
        model_id: i64,
        state: ModelDownloadState,
    ) -> Result<model_info::Model> {
        let row = self.get_model_info_entity(model_id).await?;
        let mut active_model: model_info::ActiveModel = row.into();
        active_model.downloaded = Set(state.as_downloaded());
        active_model.modify_time = Set(current_native_time());
        active_model.update(&self.db_conn).await.map_err(Into::into)
    }
}
