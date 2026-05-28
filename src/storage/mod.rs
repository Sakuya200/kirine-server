use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection};
use tracing::info;
use crate::config::EnvConfig;

mod entity;


#[derive(Clone, Debug)]
pub struct LocalStorage {
    db_conn: DatabaseConnection,
    storage_impl: StorageImpl
}

#[async_trait]
pub trait StorageService {

}

#[derive(Clone, Debug)]
pub enum StorageImpl {
    Local(LocalStorageService),
}

pub async fn init_storage(config: &EnvConfig) -> LocalStorage {
    info!("begin to init database connection");
    let db_url = config.server.db_url.clone();
    let db_user = config.server.db_user.clone();
    let db_password = config.server.db_password.clone();
    let db_name = config.server.db_name.clone();
    // 注入用户名以及密码组成连接串：postgresql://{{user}}:{{password}}@{{url}}/{{db}}
    let url = format!("postgresql://{}:{}@{}/{}", db_user, db_password, db_url, db_name);
    let conn =  Database::connect(url).await;

    if let Err(e) = conn {
        panic!("failed to connect to database: {}", e);
    }
    let conn = conn.unwrap();

    info!("successfully connected to database: {}", db_url);
    LocalStorage {
        db_conn: conn,
        storage_impl: StorageImpl::Local(LocalStorageService {}),
    }
}

#[derive(Clone, Debug)]
pub struct LocalStorageService {}
impl StorageService for LocalStorageService {

}