use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use tracing::error;
use crate::storage::LocalStorage;
use crate::storage::entity::app_meta;
use anyhow::{anyhow, bail, Result};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct UserCredential {
    pub username: String,
    pub password_hash: String,
}

#[async_trait]
pub trait AppMetaStorage {
    async fn get_app_user_credential(&self) -> Result<UserCredential>;
}

#[async_trait]
impl AppMetaStorage for LocalStorage {
    async fn get_app_user_credential(&self) -> Result<UserCredential> {
        let conn = self.db_conn.clone();
        let mut map = HashMap::<String, String>::new();
        app_meta::Entity::find()
            .select_only()
            .column(app_meta::Column::Value)
            .filter(
                app_meta::Column::Key
                    .eq("admin_username")
                    .or(app_meta::Column::Key.eq("admin_password"))
            )
            .into_tuple::<(String, String)>()
            .all(&conn)
            .await?.iter().for_each(|(key, value)| {
                map.insert(key.clone(), value.clone());
            });

        if map.is_empty() {
            error!("admin username not found in app_meta table, please check database config");
            bail!("用户相关信息不存在，请检查服务端配置是否正常");
        }

        let username = map.get("admin_username")
            .ok_or_else(|| anyhow!("用户相关信息不存在，请检查服务端配置是否正常"))?;
        let password_hash = map.get("admin_password")
            .ok_or_else(|| anyhow!("用户相关信息不存在，请检查服务端配置是否正常"))?;

        Ok(
            UserCredential {
                username: username.to_string(),
                password_hash: password_hash.to_string(),
            }
        )
    }
}