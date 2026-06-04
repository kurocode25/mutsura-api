use crate::database::Repository;
use crate::handler::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<dyn Repository + Send + Sync>,
}

pub fn create_router(db: Arc<dyn Repository + Send + Sync>) -> Router {
    // 各メソッドから共通で使用するDBの設定
    let state = AppState { db };
    // CORSの設定
    let origins = std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .split(',')
        .map(|s| s.parse::<HeaderValue>().unwrap())
        .collect::<Vec<_>>();
    let methods = [Method::POST, Method::PUT, Method::DELETE];
    let cors_layer = CorsLayer::new()
        .allow_methods(methods)
        .allow_origin(origins)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);
    // ルーターの設定
    Router::new()
        .route("/user/{user_id}", get(get_user)) // ユーザー取得
        .route("/user", post(create_user)) // ユーザー作成
        .route("/user/{user_id}", delete(delete_user)) // ユーザー削除
        .route("/post", post(create_post)) // 記事作成
        .route("/post", get(get_posts)) // 記事一覧取得
        .route("/post/{slug}", get(get_post_detail)) // 記事詳細取得
        .route("/post/{slug}", put(update_post)) // 記事更新
        .route("/post/{slug}", delete(delete_post)) // 記事削除
        .route("/category", post(create_category)) // カテゴリー登録
        .route("/category", get(get_category_list)) // カテゴリーリスト取得
        .route("/category/{slug}", get(get_category)) // カテゴリー詳細取得
        .route("/category", put(update_category)) // カテゴリー更新
        .route("/category/{slug}", delete(delete_category))
        .route("/tag", get(get_tags_for_pub_post)) // 記事用のタグリスト取得
        .route("/tag/{slug}", get(get_tag)) // タグ取得
        .route("/tag", post(create_tag)) // タグ作成
        .route("/tag/{slug}", put(update_tag)) // タグ更新
        .route("/tag/{slug}", delete(delete_tag)) // タグ削除
        .route("/page", post(create_page)) // ページ作成
        .route("/page", get(get_page_list)) // ページ一覧取得
        .route("/page/{slug}", get(get_page)) // ページ取得
        .route("/page/{slug}", put(update_page)) // ページ更新
        .route("/page/{slug}", delete(delete_page)) // ページ削除
        .route("/login", post(login)) // ログイン処理
        .route("/logout", post(logout)) // ログイン処理
        .route("/refresh", post(refresh)) // リフレッシュ処理
        .route("/admin/post", get(get_admin_posts)) // 管理画面用 記事詳細取得
        .route("/admin/post/{slug}", get(get_admin_post_detail)) // 管理画面用 記事詳細取得
        .route("/admin/page/{slug}", get(get_admin_page_detail)) // 管理画面用のページ詳細を取得
        .route("/admin/tag", get(get_tag_list)) // 管理画面用のタグリストを取得
        .route("/me", get(me)) // ログイン状態検証用
        .route("/ssg/post", get(get_ssg_post_slugs)) // SSG用 公開記事のslugリスト取得
        .route("/ssg/post-count", get(get_ssg_post_count)) // SSG用 公開記事数取得
        .route("/ssg/tag", get(get_ssg_tag_list)) // SSG用 タグのslugリストとタグを持つ公開記事の数取得
        .route("/ssg/category", get(get_ssg_category_list)) // SSG用 カテゴリーのslugリストとカテゴリーに属する公開記事の数取得
        .with_state(state)
        .layer(cors_layer)
}
