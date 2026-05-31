use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
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

    async fn update_app_user_credential(&self, credential: &UserCredential) -> Result<()>;
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

    async fn update_app_user_credential(&self, credential: &UserCredential) -> Result<()> {
        let conn = self.db_conn.clone();
        let username_active_model = app_meta::ActiveModel {
            key: Set("admin_username".to_string()),
            value: Set(credential.username.clone()),
        };
        let password_active_model = app_meta::ActiveModel {
            key: Set("admin_password".to_string()),
            value: Set(credential.password_hash.clone()),
        };

        // 使用 upsert 来保证只有一条 admin_username 和 admin_password 记录
        app_meta::Entity::insert_many(vec![username_active_model, password_active_model])
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(app_meta::Column::Key)
                    .update_columns(vec![app_meta::Column::Value])
                    .to_owned()
            )
            .exec(&conn)
            .await?;

        Ok(())
    }
}