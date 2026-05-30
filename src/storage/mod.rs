use sea_orm::{Database, DatabaseConnection};
use tracing::info;
use crate::config::EnvConfig;

mod entity;
mod app_meta_info;
mod speaker_info;

pub use app_meta_info::AppMetaStorage;
pub use app_meta_info::UserCredential;

pub use speaker_info::SpeakerInfoStorage;

#[derive(Clone, Debug)]
pub struct LocalStorage {
    db_conn: DatabaseConnection,
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
    }
}