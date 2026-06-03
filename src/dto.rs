use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// UserのDTO
///
/// # フィールド
///
/// - `id`: ユーザーの一意の識別子
/// - `name`: ユーザーの名前(日本語、英語)
/// - `email`: ユーザーのメールアドレス
/// - `password`: ユーザーのパスワード（レスポンスでは返さない）
#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Option<String>,
    pub name: NameDTO, // 英語記事と日本語記事用に対応
    pub email: String,
    pub role: String,
    pub password: String, // パスワードはレスポンスでは返さない
}

/// レスポンス用のUserDTO
#[derive(Serialize, Deserialize)]
pub struct UserResponseDTO {
    pub id: Option<String>,
    pub name: NameDTO,
    pub email: String,
    pub role: String,
}

/// 記事のリストを取得するAPIのDTO
///
/// # フィールド
///
/// - `id`: 記事の一意の識別子
/// - `title`: 記事のタイトル
/// - `slug`: 記事のスラッグ
/// - `is_draft`: 下書きかどうか
/// - `category`: 記事のカテゴリ
/// - `tags`: 記事に関連するタグのリスト
/// - `published_at`: 記事の公開日時
/// - `updated_at`: 記事の最終更新日時
/// - `image`: アイキャッチ画像
/// - `author_id`: 記事の著者のID
#[derive(Serialize, Deserialize)]
pub struct PostListDTO {
    pub id: String,        // MongoDBのドキュメントID。記事を一意に識別するためのID
    pub title: String,     // 記事のタイトル。多言語対応（日本語と英語）
    pub slug: String,      // 記事のスラッグ（URLに使われる短縮名）
    pub is_draft: bool,    // 下書きかどうか
    pub has_english: bool, // 英語の記事があるかどうか
    pub excerpt: String,   // 記事の抜粋（日本語と英語）
    pub category: ResponseCategoryDTO, // 記事のカテゴリ。カテゴリ名やスラッグを保持
    pub tags: Option<Vec<ResponseTagDTO>>, // 記事に関連するタグのリスト。タグは複数付けられる
    pub published_at: Option<DateTime<Utc>>, // 記事の公開日時。UTC形式で保存
    pub updated_at: DateTime<Utc>, // 記事の最終更新日時。UTC形式で保存
    pub image: Option<String>, // アイキャッチ画像URL
    pub author_id: String, // 記事の著者データ。IDと名前のみを返す
}

/// 記事データを登録・更新するためのPostDTO
/// 管理者用のAPIとして利用するためmodel::Postと同一構造とする
///
/// # フィールド
/// - `id`: 記事の一意の識別子
/// - `title`: 記事のタイトル
/// - `lang`: 記事の言語
/// - `slug`: 記事のスラッグ
/// - `is_draft`: 下書きかどうか
/// - `category`: 記事のカテゴリ
/// - `tags`: 記事に関連するタグのリスト
/// - `published_at`: 記事の公開日時
/// - `updated_at`: 記事の最終更新日時
/// - `author_name`: 記事の著者名
/// - `image`: アイキャッチ画像
/// - `content`: 記事の内容
#[derive(Serialize, Deserialize, Debug)]
pub struct AdminPostDTO {
    pub id: Option<String>, // MongoDBのドキュメントID。記事を一意に識別するためのID
    pub title: NameDTO,     // 記事のタイトル。多言語対応（日本語と英語）
    pub slug: String,       // 記事のスラッグ（URLに使われる短縮名）
    pub is_draft: bool,     // 下書きかどうか
    pub has_english: bool,  // 英語の記事があるかどうか
    pub category: CategoryDTO, // 記事のカテゴリ。カテゴリ名やスラッグを保持
    pub tags: Option<Vec<TagDTO>>, // 記事に関連するタグのリスト。タグは複数付けられる
    pub published_at: Option<DateTime<Utc>>, // 記事の公開日時。UTC形式で保存
    pub updated_at: Option<DateTime<Utc>>, // 記事の最終更新日時。UTC形式で保存
    pub created_at: Option<DateTime<Utc>>, // 記事の最終更新日時。UTC形式で保存
    pub author_id: Option<String>, // 記事の著者
    pub image: Option<String>, // アイキャッチ画像URL
    pub content: ContentDTO, // 記事の内容
}

/// 記事の詳細を取得するAPIのDTO
///
/// # フィールド
/// - `id`: 記事の一意の識別子
/// - `title`: 記事のタイトル
/// - `lang`: 記事の言語
/// - `slug`: 記事のスラッグ
/// - `is_draft`: 下書きかどうか
/// - `category`: 記事のカテゴリ
/// - `tags`: 記事に関連するタグのリスト
/// - `published_at`: 記事の公開日時
/// - `updated_at`: 記事の最終更新日時
/// - `author_name`: 記事の著者名
/// - `image`: アイキャッチ画像
/// - `content`: 記事の内容
/// - `excerpt`: 記事の抜粋
#[derive(Serialize, Deserialize, Debug)]
pub struct PostDetailDTO {
    pub id: Option<String>, // MongoDBのドキュメントID。記事を一意に識別するためのID
    pub title: String,      // 記事のタイトル。多言語対応（日本語と英語）
    pub lang: String,       // 記事の言語
    pub slug: String,       // 記事のスラッグ（URLに使われる短縮名）
    pub is_draft: bool,     // 下書きかどうか
    pub has_english: bool,  // 英語の記事があるかどうか
    pub category: ResponseCategoryDTO, // 記事のカテゴリ。カテゴリ名やスラッグを保持
    pub tags: Option<Vec<ResponseTagDTO>>, // 記事に関連するタグのリスト。タグは複数付けられる
    pub published_at: Option<DateTime<Utc>>, // 記事の公開日時。UTC形式で保存
    pub updated_at: DateTime<Utc>, // 記事の最終更新日時。UTC形式で保存
    pub author: Option<AuthorDTO>, // 記事の著者
    pub image: Option<String>, // アイキャッチ画像URL
    pub content: String,    // 記事の内容
    pub excerpt: String,    // 記事の抜粋
}

// ページネーションの情報を保持するDTO
#[derive(Serialize, Deserialize)]
pub struct PaginationDTO {
    pub page: i64,               // 現在のページ番号
    pub per_page: i64,           // 1ページあたりの表示数
    pub total: i64,              // 全体のデータ数
    pub total_pages: i64,        // 全体のページ数
    pub has_next_page: bool,     // 次のページが存在するかどうか
    pub has_previous_page: bool, // 前のページが存在するかどうか
}

impl PaginationDTO {
    pub fn new(total: i64, limit: i64, page: i64) -> PaginationDTO {
        let total_pages = if total % limit > 0 {
            total / limit + 1
        } else {
            total / limit
        };
        let has_next_page: bool = total_pages > page;
        let has_previous_page: bool = page > 1;
        PaginationDTO {
            page,
            per_page: limit,
            total,
            total_pages,
            has_next_page,
            has_previous_page,
        }
    }
}

// 記事一覧取得APIのレスポンスDTO
#[derive(Serialize, Deserialize)]
pub struct PostListResponseDTO {
    pub posts: Vec<PostListDTO>,   // 記事のリスト
    pub pagination: PaginationDTO, // ページネーション情報
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorDTO {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NameDTO {
    pub ja: String, // 日本語でのタイトル
    pub en: String, // 英語でのタイトル
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDTO {
    pub ja: String, // 日本語の記事
    pub en: String, // 英語の記事
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryDTO {
    pub slug: String,  // カテゴリのスラッグ（URLに使われる短縮名）
    pub name: NameDTO, // カテゴリ名（多言語対応）
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagDTO {
    pub slug: String,  // タグのスラッグ（URLに使われる短縮名）
    pub name: NameDTO, // タグ名（多言語対応）
}

// レスポンス用のカテゴリーDTO
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCategoryDTO {
    pub slug: String, // カテゴリのスラッグ（URLに使われる短縮名）
    pub name: String, // カテゴリ名
}

// レスポンス用のタグDTO
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTagDTO {
    pub slug: String, // タグのスラッグ（URLに使われる短縮名）
    pub name: String, // タグ名
}

/// Pageリクエスト用のDTO
#[derive(Serialize, Deserialize, Debug)]
pub struct PageDTO {
    pub slug: String,
    pub title: NameDTO,
    pub content: ContentDTO,
}

/// ページ一覧取得用のDTO
/// リスト表示に必要な最小限の情報のみを含む
#[derive(Serialize, Deserialize)]
pub struct PageListDTO {
    pub id: String,
    pub slug: String,
    pub title: NameDTO,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ページ一詳細データのレスポンス用DTO
#[derive(Serialize, Deserialize)]
pub struct PageDetailDTO {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 管理画面用のページ詳細レスポンスDTO
#[derive(Serialize, Deserialize)]
pub struct AdminPageDetailDTO {
    pub id: String,
    pub slug: String,
    pub title: NameDTO,
    pub content: ContentDTO,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// SSG用のカウント付きデータレスポンスのDTO
#[derive(Serialize, Deserialize)]
pub struct SSGDataWithCountDTO {
    pub slug: String,
    pub count: i64,
}

// SSG用のPost数データレスポンスのDTO
#[derive(Serialize, Deserialize)]
pub struct SSGDataPostCountDTO {
    pub count: i64,
}
