use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;

// ユーザーの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDBのドキュメントID。ユーザーを一意に識別するためのID
    pub name: Name,            // ユーザー名(日本語と英語)
    pub email: String,         // ユーザーのメールアドレス
    pub password_hash: String, // ユーザーのパスワードのハッシュ（平文パスワードではない）
    pub role: String,          // ユーザーの役割（例：admin, user）
    pub created_at: DateTime,  // ユーザーの作成日時
    pub updated_at: DateTime,  // ユーザーの最終更新日時
}

// 記事の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDBのドキュメントID。記事を一意に識別するためのID
    pub title: Name,                    // 記事のタイトル。多言語対応（日本語と英語）
    pub slug: String,                   // 記事のスラッグ（URLに使われる短縮名）
    pub is_draft: bool,                 // 下書きかどうか
    pub has_english: bool,              // 英語の記事があるかどうか
    pub category: Category,             // 記事のカテゴリ。カテゴリ名やスラッグを保持
    pub tags: Option<Vec<Tag>>,         // 記事に関連するタグのリスト。タグは複数付けられる
    pub content: Content,               // 記事のコンテンツ。多言語対応（日本語と英語）
    pub published_at: Option<DateTime>, // 記事の公開日時。UTC形式で保存
    pub updated_at: DateTime,           // 記事の最終更新日時。UTC形式で保存
    pub created_at: DateTime,           // 記事の作成日時。UTC形式で保存
    pub image: Option<String>,          // アイキャッチ画像URL
    pub author_id: ObjectId,            // 記事の著者ID
}

// 記事詳細の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct PostDetail {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDBのドキュメントID。記事を一意に識別するためのID
    pub title: Name,                    // 記事のタイトル。多言語対応（日本語と英語）
    pub slug: String,                   // 記事のスラッグ（URLに使われる短縮名）
    pub is_draft: bool,                 // 下書きかどうか
    pub has_english: bool,              // 英語の記事があるかどうか
    pub category: Category,             // 記事のカテゴリ。カテゴリ名やスラッグを保持
    pub tags: Option<Vec<Tag>>,         // 記事に関連するタグのリスト。タグは複数付けられる
    pub content: Content,               // 記事のコンテンツ。多言語対応（日本語と英語）
    pub published_at: Option<DateTime>, // 記事の公開日時。UTC形式で保存
    pub updated_at: DateTime,           // 記事の最終更新日時。UTC形式で保存
    pub created_at: DateTime,           // 記事の作成日時。UTC形式で保存
    pub image: Option<String>,          // アイキャッチ画像URL
    pub author: Option<Author>,         // 記事の著者
}

// 記事が保持する著者データの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: Name,    // 著者名：ユーザー名と関連付けられる
    pub email: String, // ユーザーのメールアドレス
}

// 記事合計数の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Total {
    pub count: i64, // 合計数
}

// 記事合計と記事の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithTotal {
    pub data: Vec<Post>,   // 記事のリスト
    pub total: Vec<Total>, // 記事の合計数
}

// タイトルの構造体（多言語対応）
#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub ja: String, // 日本語でのタイトル
    pub en: String, // 英語でのタイトル
}

// カテゴリの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub slug: String, // カテゴリのスラッグ（URLに使われる短縮名）
    pub name: Name,   // カテゴリ名（多言語対応）
}

// タグの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub slug: String, // タグのスラッグ（URLに使われる短縮名）
    pub name: Name,   // タグ名（多言語対応）
}

// コンテンツの構造体（多言語対応）
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub ja: String, // 日本語でのコンテンツ
    pub en: String, // 英語でのコンテンツ
}

// タグと記事数の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct TagWithCount {
    pub slug: String,
    pub name: Name,
    pub count: i64,
}

// カテゴリーと記事数の構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryWithCount {
    pub slug: String,
    pub name: Name,
    pub count: i64,
}

// ページの構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDBのドキュメントID。ページを一意に識別するためのID
    pub title: Name,          // ページのタイトル（日本語と英語）
    pub slug: String,         // ページのスラッグ（URLに使われる短縮名）
    pub content: Content,     // ページのコンテンツ（日本語と英語）
    pub created_at: DateTime, // ページの作成日時
    pub updated_at: DateTime, // ページの最終更新日時
}
