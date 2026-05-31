use dotenv::dotenv;
use mongodb::bson::DateTime;
use mongodb::{Client, Collection, Database};
use password_auth::generate_hash;
use std::env;
use std::sync::Arc;

use crate::api::create_router;
use crate::model::{Name, User};

// 管理者ユーザー追加処理
pub async fn add_admin(
    j_name: String,
    e_name: String,
    email: String,
    password: String,
) -> mongodb::error::Result<()> {
    // 環境変数からMongoDBのURIを取得
    dotenv().ok();
    let mongodb_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let database_name = env::var("MONGODB_NAME").unwrap_or_else(|_| "blog".to_string());

    // DBの準備
    let client: Client = Client::with_uri_str(mongodb_uri).await.unwrap();
    let col: Collection<User> = client.database(&database_name).collection::<User>("users");

    // ユーザー情報を作成
    let user = User {
        id: None,
        name: Name {
            ja: j_name,
            en: e_name,
        },
        email,
        password_hash: generate_hash(&password),
        role: "admin".to_string(),
        updated_at: DateTime::now(),
        created_at: DateTime::now(),
    };
    col.insert_one(user).await?;

    Ok(())
}

// サーバー機能を提供
pub async fn serve() {
    // 環境変数からMongoDBのURIを取得
    dotenv().ok();
    let mongodb_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let database_name = env::var("MONGODB_NAME").unwrap_or_else(|_| "blog".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    // MongoDBのクライアントを作成
    let client: Client = Client::with_uri_str(mongodb_uri).await.unwrap();
    let db: Database = client.database(&database_name);
    // サーバーの実行
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, create_router(Arc::new(db)))
        .await
        .unwrap();
}
