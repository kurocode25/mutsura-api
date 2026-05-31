use async_trait::async_trait;
use mockall::automock;
use mongodb::{bson::Document, error::Result};

#[automock]
#[async_trait]
pub trait Database {
    async fn find_one(&self, collection: &str, filter: Document) -> Result<Option<Document>>;
    // 他のデータベース操作メソッド...
}
