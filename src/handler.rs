use crate::auth::Claims;
use crate::dto::*;
use crate::mapper::*;
use crate::model::*;
use crate::utils::deserialize_option_i64_from_str;
use crate::{api::AppState, auth::check_refresh_token};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use mongodb::bson::DateTime;
use mongodb::bson::{doc, oid::ObjectId, Document};
use password_auth::{generate_hash, verify_password};
use serde::Deserialize;
use time::Duration;

// ユーザー作成
pub async fn create_user(
    _claims: Claims,
    State(state): State<AppState>,
    Json(user_dto): Json<UserDTO>,
) -> impl IntoResponse {
    //{{{
    // ユーザー情報を作成
    let user = User {
        id: None,
        name: Name {
            ja: user_dto.name.ja,
            en: user_dto.name.en,
        },
        email: user_dto.email,
        password_hash: generate_hash(&user_dto.password),
        role: user_dto.role,
        updated_at: DateTime::now(),
        created_at: DateTime::now(),
    };
    // DBにユーザーを登録
    match state.db.create_user(user).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } //}}}
}

// ユーザー取得（ID指定）
pub async fn get_user(
    _claims: Claims,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let id = match ObjectId::parse_str(&user_id) {
        //{{{
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    match state.db.find_user(&id).await {
        // TODO: 取得したユーザーデータをDTOにマッピング
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } //}}}
}

// ユーザー更新
pub async fn update_user(
    State(_state): State<AppState>,
    Json(_user_dto): Json<UserDTO>,
    _claims: Claims,
) -> impl IntoResponse {
    // 更新するユーザー情報を作成

    // データ更新
}

// ユーザー削除
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _claims: Claims,
) -> impl IntoResponse {
    let id = match ObjectId::parse_str(&user_id) {
        // {{{
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    match state.db.delete_user(id).await {
        Ok(res) => {
            if res.deleted_count >= 1 {
                StatusCode::NO_CONTENT.into_response() // 成功時：204を返す
            } else {
                StatusCode::NOT_FOUND.into_response() // 削除するものが見つからない場合：404を返す
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // 失敗時：500エラーを返す
    } // }}}
}

// 記事作成
pub async fn create_post(
    claims: Claims,
    State(state): State<AppState>,
    Json(post_dto): Json<AdminPostDTO>,
) -> impl IntoResponse {
    // ユーザー情報を取得
    let author_id = match ObjectId::parse_str(claims.sub) {
        //{{{
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    // 記事情報を作成
    let post = Post {
        id: None,
        title: Name {
            ja: post_dto.title.ja,
            en: post_dto.title.en,
        },
        slug: post_dto.slug,
        is_draft: post_dto.is_draft,
        has_english: post_dto.has_english,
        category: to_category(&post_dto.category),
        tags: to_tags(post_dto.tags),
        published_at: {
            // 下書きの場合は公開日時を設定しない
            if post_dto.is_draft {
                None
            } else {
                Some(DateTime::now())
            }
        },
        updated_at: DateTime::now(),
        created_at: DateTime::now(),
        author_id,
        image: post_dto.image,
        content: Content {
            ja: post_dto.content.ja,
            en: post_dto.content.en,
        },
    };
    // DBに記事を登録
    match state.db.create_post(post).await {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } //}}}
}

/// Retrieves a list of posts based on the provided query parameters.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection.
/// * `params` - The query parameters for filtering and paginating the list of posts.
///
/// # Returns
///
/// An HTTP response containing the list of posts in JSON format. If no posts are found,
/// a 404 Not Found status is returned. If an error occurs during the database query,
/// a 500 Internal Server Error status is returned.
///
pub async fn get_posts(
    State(state): State<AppState>,
    Query(params): Query<PostListQueryParams>,
) -> impl IntoResponse {
    let lang = match params.common.lang {
        //{{{
        Some(ref l) => match l.as_str() {
            "ja" => Lang::Ja,
            "en" => Lang::En,
            _ => return StatusCode::BAD_REQUEST.into_response(),
        },
        None => Lang::Ja,
    };
    let limit = params.common.limit.unwrap_or(10); // 1ページあたりのデータ数
    let page = params.common.page.unwrap_or(1); // ページ番号
    let filter = create_public_postlist_filter(
        &params.common.category,
        &params.common.tag,
        &params.common.lang,
    );
    match state
        .db
        .find_posts(create_pipeline(&params.common, &filter))
        .await
    {
        Ok(pwt) => common_get_posts_response(pwt, limit, page, &lang),
        Err(e) => {
            println!("error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    } //}}}
}

// 管理用のポスト取得API
pub async fn get_admin_posts(
    _claims: Claims,
    State(state): State<AppState>,
    Query(params): Query<AdminPostListQueryParams>,
) -> impl IntoResponse {
    let lang = match params.common.lang {
        //{{{
        Some(ref l) => match l.as_str() {
            "ja" => Lang::Ja,
            "en" => Lang::En,
            _ => return StatusCode::BAD_REQUEST.into_response(),
        },
        None => Lang::Ja,
    };
    let limit = params.common.limit.unwrap_or(10); // 1ページあたりのデータ数
    let page = params.common.page.unwrap_or(1); // ページ番号
    let filter = create_admin_postlist_filter(
        &params.common.category,
        &params.common.tag,
        &params.common.lang,
        &params.status,
    );
    match state
        .db
        .find_posts(create_pipeline(&params.common, &filter))
        .await
    {
        Ok(pwt) => common_get_posts_response(pwt, limit, page, &lang),
        Err(e) => {
            println!("error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    } //}}}
}

// 公開用、管理用共通の記事一覧データ取得処理
fn common_get_posts_response(pwt: PostWithTotal, limit: i64, page: i64, lang: &Lang) -> Response {
    let posts = pwt.data;
    let total = if let Some(t) = pwt.total.get(0) {
        t.count
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let pagination: PaginationDTO = PaginationDTO::new(total, limit, page);
    let post_lists: Vec<PostListDTO> = posts
        .into_iter()
        .map(|post| to_post_list_dto(post, lang))
        .collect();

    (
        StatusCode::OK,
        Json(PostListResponseDTO {
            posts: post_lists,
            pagination,
        }),
    )
        .into_response()
}

// 記事一覧取得の共通パラメータ
#[derive(Deserialize)]
struct CommonPostListQueryParams {
    category: Option<String>, // カテゴリ
    tag: Option<String>,      // タグ
    lang: Option<String>,     // 言語
    #[serde(default, deserialize_with = "deserialize_option_i64_from_str")]
    limit: Option<i64>, // 取得件数
    #[serde(default, deserialize_with = "deserialize_option_i64_from_str")]
    page: Option<i64>, // ページ
    sort: Option<String>,     // ソート
    q: Option<String>,        // 検索クエリ
}

// 記事一覧データを取得時のクエリパラメータ
#[derive(Deserialize)]
pub struct PostListQueryParams {
    #[serde(flatten)]
    common: CommonPostListQueryParams,
}

// 記事のステータス
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PostStatus {
    Draft,
    Published,
}

// 管理者用の記事取得クエリパラメータ
#[derive(Deserialize)]
pub struct AdminPostListQueryParams {
    #[serde(flatten)]
    common: CommonPostListQueryParams,
    status: Option<PostStatus>,
}

fn create_common_postlist_filter(
    category: &Option<String>,
    tag: &Option<String>,
    lang: &Option<String>,
) -> Document {
    // filterを作成
    let mut filter = doc! {};
    // カテゴリ指定がある場合はクエリに追加
    if let Some(category) = &category {
        filter.insert("category.slug", category);
    }
    // タグ指定がある場合はクエリに追加
    if let Some(tag) = &tag {
        filter.insert("tags.slug", tag);
    }

    // 言語に英語が指定された場合は`has_english=true`のみを返す
    if let Some(ref l) = lang {
        if l == "en" {
            filter.insert("has_english", true);
        };
    };

    filter
}
// 公開用フィルター作成
fn create_public_postlist_filter(
    category: &Option<String>,
    tag: &Option<String>,
    lang: &Option<String>,
) -> Document {
    let mut filter = create_common_postlist_filter(category, tag, lang);
    // ドラフトは非表示
    filter.insert("is_draft", false);

    filter
}
// 管理者用フィルター作成
fn create_admin_postlist_filter(
    category: &Option<String>,
    tag: &Option<String>,
    lang: &Option<String>,
    status: &Option<PostStatus>, // draft, publish
) -> Document {
    let mut filter = create_common_postlist_filter(category, tag, lang);

    // statusに応じたfilterの処理
    if let Some(st) = status {
        match st {
            PostStatus::Draft => {
                filter.insert("is_draft", true);
            }
            PostStatus::Published => {
                filter.insert("is_draft", false);
            }
        }
    }

    filter
}

// 記事一覧取得のためのパイプラインを生成
fn create_pipeline(params: &CommonPostListQueryParams, filter: &Document) -> Vec<Document> {
    let limit = params.limit.unwrap_or(10); //{{{
    let page = params.page.unwrap_or(1);
    let skip: i64 = (page - 1) * limit;
    let sort = params.sort.as_deref().unwrap_or("updated_at:desc");
    let sort = sort.split(":").collect::<Vec<&str>>();
    let sort_key = sort[0];
    let sort_value = match sort[1] {
        "asc" => 1,
        "desc" => -1,
        _ => 1,
    };

    let mut pipeline: Vec<Document> = Vec::new();
    //}}}

    // パイプラインを作成
    // 検索クエリがある場合はクエリに追加
    if let Some(q) = &params.q {
        let regex_val = mongodb::bson::Regex {
            pattern: q.clone(),
            options: "i".to_string(),
        };
        pipeline.push(doc! {
            "$match": {
                "$or": [
                    { "title.ja": { "$regex": &regex_val } },
                    { "title.en": { "$regex": &regex_val } },
                    { "content.ja": { "$regex": &regex_val } },
                    { "content.en": { "$regex": &regex_val } }
                ]
            }
        });
    }

    pipeline.push(doc! {
        "$facet" : {
            "data" : [
                doc! { "$match" : filter},
                doc! { "$sort" : {sort_key : sort_value}},
                doc! { "$skip" : skip},
                doc! { "$limit" : limit},
            ],
            "total" :[
                doc! { "$match" : filter},
                doc! { "$count" : "count"}
            ]
        }
    });

    pipeline
}

// 記事詳細取得（slug指定）
pub async fn get_post_detail(
    State(state): State<AppState>, //{{{
    Path(slug): Path<String>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let lang = match params.lang {
        Some(ref l) => match l.as_str() {
            "ja" => Lang::Ja,
            "en" => Lang::En,
            _ => return StatusCode::BAD_REQUEST.into_response(),
        },
        None => Lang::Ja,
    };
    match state.db.find_post_detail(slug).await {
        // レスポンス処理
        Ok(Some(post)) => {
            // レスポンス用のデータに変換
            let post_dtail = to_post_dto(post, &lang);
            (StatusCode::OK, Json(post_dtail)).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } //}}}
}

// 管理画面用 記事詳細取得
pub async fn get_admin_post_detail(
    _claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.db.find_post(&slug).await {
        // レスポンス処理
        Ok(Some(post)) => {
            // レスポンス用のデータに変換
            let post_dtail = to_admin_post_dto(post);
            (StatusCode::OK, Json(post_dtail)).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct Params {
    lang: Option<String>,
}

// 記事更新
pub async fn update_post(
    claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(post_dto): Json<AdminPostDTO>,
) -> impl IntoResponse {
    // 既存の記事データを取得{{{
    let existing_post = match state.db.find_post(&slug).await {
        Ok(Some(post)) => post,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let author_id = match ObjectId::parse_str(claims.sub) {
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    // 公開日の決定ロジック
    let published_at = if post_dto.is_draft {
        // 下書きの場合は公開日を設定しない
        None
    } else {
        // 既に公開日がある場合はその値を維持し、ない場合は現在時刻を設定
        existing_post.published_at.or_else(|| Some(DateTime::now()))
    };

    // 更新する記事情報を作成
    let post = Post {
        id: existing_post.id,
        title: Name {
            ja: post_dto.title.ja,
            en: post_dto.title.en,
        },
        slug: post_dto.slug,
        is_draft: post_dto.is_draft,
        has_english: post_dto.has_english,
        category: to_category(&post_dto.category),
        tags: to_tags(post_dto.tags),
        published_at,
        updated_at: DateTime::now(),
        created_at: existing_post.created_at, // 作成日は元の値を維持
        author_id,
        image: post_dto.image,
        content: Content {
            ja: post_dto.content.ja,
            en: post_dto.content.en,
        },
    };

    // DBの記事を更新
    match state.db.update_post(&slug, &post).await {
        Ok(Some(updated_post)) => {
            (StatusCode::OK, Json(to_admin_post_dto(updated_post))).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } // }}}
}

// 記事削除(slug指定)
pub async fn delete_post(
    _claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.db.delete_post(slug).await {
        Ok(res) => {
            if res.deleted_count >= 1 {
                StatusCode::NO_CONTENT.into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// 新しいカテゴリを作成し、データベースに登録する非同期関数
///
/// この関数は、`AppState`に格納されているデータベースインスタンスを使用して、
/// `CategoryDTO`からカテゴリ情報を取得し、新しいカテゴリをデータベースに登録します。
///
/// # 引数
/// - `state`: `AppState`型の状態情報。データベース接続などの情報を保持しています。
/// - `category_dto`: `CategoryDTO`型のカテゴリ情報を格納したDTO。これを元に新しいカテゴリが作成されます。
///
/// # 戻り値
/// 成功時は新しいカテゴリの情報と共にHTTPステータス`201 Created`を返します。
/// 失敗時はHTTPステータス`500 Internal Server Error`を返します。
///
/// # エラー
/// データベースへの登録に失敗した場合、`500`ステータスが返されます。
pub async fn create_category(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Json(category_dto): Json<CategoryDTO>, // クライアントから送られたカテゴリ情報を含むDTO
) -> impl IntoResponse {
    // カテゴリー情報を作成{{{
    let category = Category {
        slug: category_dto.slug,
        name: Name {
            ja: category_dto.name.ja,
            en: category_dto.name.en,
        },
    };

    // データの登録
    match state.db.create_caregory(category).await {
        // データベースへの登録成功時
        Ok(category) => (StatusCode::CREATED, Json(category)).into_response(),
        // データベースへの登録失敗時
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    } //}}}
}

// カテゴリー詳細を取得
pub async fn get_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.db.find_category(slug).await {
        Ok(res) => match res {
            Some(category) => (StatusCode::OK, Json(category)).into_response(),
            None => (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// カテゴリー情報を更新するAPIエンドポイント
///
/// この関数は、`PUT`または`PATCH`リクエストで受け取ったカテゴリ情報を使用して、
/// データベース内の既存のカテゴリーを更新します。
///
/// # 引数
///
/// - `state`: アプリケーションの状態を表す構造体。データベース接続などの情報を含みます。
/// - `category_dto`: クライアントから送信されたカテゴリ情報を持つDTO（データ転送オブジェクト）。
///   この情報には、カテゴリ名やスラッグなどの更新情報が含まれています。
///
/// # 戻り値
///
/// 更新が成功した場合は、HTTPステータスコード200（OK）とともに更新されたカテゴリ情報を
/// JSON形式で返します。失敗した場合は、HTTPステータスコード500（内部サーバーエラー）を返します。
///
/// # エラーハンドリング
///
/// - 更新処理が成功した場合、更新後のカテゴリ情報をJSONとして返します。
/// - 何らかのエラーが発生した場合、HTTPステータスコード500を返し、エラーメッセージは含みません。
///
/// # パフォーマンス
/// この操作は非同期で実行され、データベースへのアクセスを含むため、
/// サーバーの負荷やネットワーク遅延によってパフォーマンスに影響を与える可能性があります。
///
/// # 注意点
/// - `slug`は一意である必要があり、同じ`slug`を持つカテゴリーがデータベース内に存在しないことを確認してください。
pub async fn update_category(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Json(category_dto): Json<CategoryDTO>, // クライアントから送られたカテゴリ情報を含むDTO
) -> impl IntoResponse {
    // カテゴリDTOを基に新しいカテゴリー構造体を作成{{{
    let new_category = to_category(&category_dto);
    let filter = &category_dto.slug;

    // カテゴリー情報の更新処理
    match state.db.update_category(&filter, &new_category).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(), // 成功時：更新されたカテゴリ情報を返す
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // 失敗時：500エラーを返す
    } //}}}
}

/// 指定されたカテゴリを削除する非同期関数
///
/// この関数は、指定されたカテゴリをデータベースから削除します。削除が成功した場合は、
/// HTTPステータスコード204（No Content）を返します。削除対象のカテゴリが見つからない場合は、
/// HTTPステータスコード404（Not Found）を返し、削除処理に失敗した場合は、
/// HTTPステータスコード500（Internal Server Error）を返します。
///
/// # 引数
///
/// - `state`: アプリケーションの状態情報を保持する`AppState`。これにはデータベース接続やその他の必要な状態が含まれています。
/// - `slug`: 削除対象のカテゴリを一意に識別する文字列。パスパラメータとして指定されます。
///
/// # 戻り値
///
/// - 成功時: `StatusCode::NO_CONTENT`（HTTP 204）。
/// - 削除対象のカテゴリが見つからない場合: `StatusCode::NOT_FOUND`（HTTP 404）。
/// - 処理中にエラーが発生した場合: `StatusCode::INTERNAL_SERVER_ERROR`（HTTP 500）。
///
/// # 処理の流れ
///
/// 1. 指定されたカテゴリの削除をデータベースで試みます。
/// 2. 削除が成功した場合、`deleted_count`が1以上であれば204（成功）を返します。
/// 3. `deleted_count`が0の場合は、カテゴリが存在しなかったことを示すため404を返します。
/// 4. 削除処理中にエラーが発生した場合は500（内部サーバーエラー）を返します。
///
pub async fn delete_category(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Path(slug): Path<String>,      // 削除対象のカテゴリを特定するslug
) -> impl IntoResponse {
    match state.db.delete_category(slug).await {
        // {{{
        Ok(res) => {
            if res.deleted_count >= 1 {
                StatusCode::NO_CONTENT.into_response() // 成功時：204を返す
            } else {
                StatusCode::NOT_FOUND.into_response() // 削除するものが見つからない場合：404を返す
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // 失敗時：500エラーを返す
    } // }}}
}

// タグ作成
pub async fn create_tag(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Json(tag_dto): Json<TagDTO>,   // クライアントから送られたカテゴリ情報を含むDTO
) -> impl IntoResponse {
    // タグ情報を生成
    let new_tag = Tag {
        slug: tag_dto.slug,
        name: Name {
            ja: tag_dto.name.ja,
            en: tag_dto.name.en,
        },
    };

    match state.db.create_tag(&new_tag).await {
        Ok(_) => (StatusCode::CREATED, Json(new_tag)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
// 管理画面用タグリスト取得
pub async fn get_tag_list(State(state): State<AppState>, _claims: Claims) -> impl IntoResponse {
    match state.db.find_tag_list().await {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// 公開記事用のタグリスト取得
pub async fn get_tags_for_pub_post(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or("ja".to_string());

    // パイプラインの準備
    let mut pipeline: Vec<Document> = Vec::new();
    if lang != "ja" {
        pipeline.push(doc! { "$match": { "is_draft": false, "has_english": true } });
    } else {
        pipeline.push(doc! { "$match": { "is_draft": false } });
    }

    let mut add_items: Vec<Document> = vec![
        doc! { "$unwind": "$tags" },
        doc! {
            "$group": {
                "_id": "$tags.slug",
                "name": { "$first": "$tags.name" }
            }
        },
        doc! {
            "$project": {
                "_id": 0,
                "slug": "$_id",
                "name": 1
            }
        },
    ];
    pipeline.append(&mut add_items);

    // タグリストの取得
    match state.db.find_tags_for_pub_post(pipeline).await {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(e) => {
            println!("error message: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// タグ取得
pub async fn get_tag(
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Path(slug): Path<String>,      // 削除対象のカテゴリを特定するslug
) -> impl IntoResponse {
    match state.db.find_tag(slug).await {
        Ok(tag) => (StatusCode::OK, Json(tag)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
// タグ更新
pub async fn update_tag(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Path(slug): Path<String>,      // 削除対象のカテゴリを特定するslug
    Json(tag_dto): Json<TagDTO>,   // クライアントから送られたTag情報を含むDTO
) -> impl IntoResponse {
    let new_tag: Tag = Tag {
        slug: tag_dto.slug,
        name: Name {
            ja: tag_dto.name.ja,
            en: tag_dto.name.en,
        },
    };
    match state.db.update_tag(&slug, &new_tag).await {
        Ok(_) => (StatusCode::OK, Json(new_tag)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// タグ削除
pub async fn delete_tag(
    _claims: Claims,
    State(state): State<AppState>, // アプリケーションの状態情報（データベースなど）
    Path(slug): Path<String>,      // 削除対象のカテゴリを特定するslug
) -> impl IntoResponse {
    match state.db.delete_tag(slug).await {
        Ok(res) => {
            if res.deleted_count >= 1 {
                StatusCode::NO_CONTENT.into_response() // 削除成功時は204を返す
            } else {
                StatusCode::NOT_FOUND.into_response() // 削除対象がない場合は404を返す
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

use crate::auth::{issue_jwt, issue_refresh_token};
use axum::http::header;
use axum_extra::extract::cookie::{Cookie, SameSite};

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

// ログイン処理
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // emailとパスワードを受け取りDBからユーザー情報を取得し照合
    let user = match state.db.find_user_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        // ユーザーが見つからない場合、またはDBエラー
        _ => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // パスワードを照合
    match verify_password(&payload.password, &user.password_hash) {
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(_) => (),
    };

    // 照合に成功したらJWTとrefresh tokenを発行
    let id = match user.id {
        Some(id) => id.to_string(),
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let jwt = match issue_jwt(&id) {
        Ok(t) => t,
        Err(e) => return e.into_response(),
    };
    let refresh_token = issue_refresh_token(&id);
    let refresh_token_for_cookie = refresh_token.token.clone();

    // refresh tokenをDBに登録
    if state.db.create_refresh_token(refresh_token).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // refresh tokenはHttpOnly Cookieで返す。
    return create_token_response(&jwt, &refresh_token_for_cookie);
}

// ログアウト処理
pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    // リフレッシュトークンの無効化
    let old_token = match jar.get("refresh_token") {
        Some(cookie) => cookie.value(),
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if let Err(_) = state.db.revoke_refresh_token(&old_token).await {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    // Cookieの削除
    let cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    // JSON形式でJWTを返す。
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    (headers, StatusCode::NO_CONTENT).into_response()
}

// リフレッシュトークンの処理
pub async fn refresh(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    // DBからrefresh_tokenの取得
    let old_token = match jar.get("refresh_token") {
        Some(cookie) => cookie.value(),
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let refresh_token = match state.db.find_refresh_token(&old_token).await {
        Ok(Some(t)) => t,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    // 受け取ったリフレッシュトークンの認証
    if check_refresh_token(&refresh_token) {
        // 古いリフレッシュトークンの無効化
        let _ = state.db.revoke_refresh_token(&refresh_token.token).await;

        // 認証に成功した場合はJWTを発行しrefresh tokenを再発行
        let refresh_token = issue_refresh_token(&refresh_token.user_id);
        let refresh_token_for_cookie = refresh_token.token.clone();
        let id = refresh_token.user_id.clone();
        // refresh tokenをDBに登録
        if state.db.create_refresh_token(refresh_token).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        // JWTとリフレッシュトークンを返す
        let jwt = match issue_jwt(&id) {
            Ok(t) => t,
            Err(e) => return e.into_response(),
        };

        // JWTとリフレッシュトークンを返す
        create_token_response(&jwt, &refresh_token_for_cookie)
    } else {
        // 認証に失敗した場合は401エラーを返す。
        return StatusCode::UNAUTHORIZED.into_response();
    }
}

// JWTをリフレッシュトークンを返すレスポンス作成
fn create_token_response(token: &str, refresh: &str) -> Response<Body> {
    // refresh tokenはHttpOnly Cookieで返す。
    let cookie = Cookie::build(("refresh_token", refresh))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    // JSON形式でJWTを返す。
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    (headers, Json(serde_json::json!({ "token": token }))).into_response()
}

//
pub async fn me(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    // User情報の取得
    let user_id: ObjectId = match ObjectId::parse_str(claims.sub) {
        Ok(id) => id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let user = match state.db.find_user(&user_id).await {
        Ok(Some(u)) => u,
        _ => {
            println!("User get Error");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    (Json(serde_json::json!({ "user": to_user_response_dto(&user) }))).into_response()
}

// SSG用 公開記事のslugリスト取得
pub async fn get_ssg_post_slugs(
    State(state): State<AppState>,
    Query(params): Query<SsgQueryParams>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or("ja".to_string());
    match state.db.find_ssg_post_slugs(&lang).await {
        Ok(slugs) => (StatusCode::OK, Json(slugs)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// SSG用 公開記事数取得
pub async fn get_ssg_post_count(
    State(state): State<AppState>,
    Query(params): Query<SsgQueryParams>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or("ja".to_string());
    match state.db.find_ssg_post_count(&lang).await {
        Ok(count) => (StatusCode::OK, Json(SSGDataPostCountDTO { count })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// SSG用 タグのslugリストとタグを持つ公開記事の数取得
pub async fn get_ssg_tag_list(
    State(state): State<AppState>,
    Query(params): Query<SsgQueryParams>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or("ja".to_string());
    match state.db.find_ssg_tag_list(&lang).await {
        Ok(tags) => (StatusCode::OK, Json(to_ssg_data_with_count_dto(tags))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// SSG用 カテゴリーのslugリストとカテゴリーに属する公開記事の数取得
pub async fn get_ssg_category_list(
    State(state): State<AppState>,
    Query(params): Query<SsgQueryParams>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or("ja".to_string());
    match state.db.find_ssg_category_list(&lang).await {
        Ok(categories) => {
            (StatusCode::OK, Json(to_ssg_data_with_count_dto(categories))).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct SsgQueryParams {
    lang: Option<String>,
}

#[derive(Deserialize)]
pub struct CategoryListQueryParams {
    #[serde(rename = "include-count")]
    include_count: Option<bool>,
}

// カテゴリー一覧を取得
pub async fn get_category_list(
    State(state): State<AppState>,
    Query(params): Query<CategoryListQueryParams>,
) -> impl IntoResponse {
    if params.include_count.unwrap_or(false) {
        match state.db.find_category_list_with_count().await {
            Ok(category_list) => (StatusCode::OK, Json(category_list)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        match state.db.find_category_list().await {
            Ok(category_list) => (StatusCode::OK, Json(category_list)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

// ページ作成
pub async fn create_page(
    _claims: Claims,
    State(state): State<AppState>,
    Json(page_dto): Json<PageDTO>,
) -> impl IntoResponse {
    let now = DateTime::now();
    let page = Page {
        id: None,
        title: Name {
            ja: page_dto.title.ja,
            en: page_dto.title.en,
        },
        slug: page_dto.slug.clone(),
        content: Content {
            ja: page_dto.content.ja,
            en: page_dto.content.en,
        },
        created_at: now,
        updated_at: now,
    };
    let slug = page_dto.slug.clone();
    match state.db.create_page(page).await {
        Ok(_) => match state.db.find_page(&slug).await {
            Ok(Some(created_page)) => (StatusCode::CREATED, Json(created_page)).into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ページ取得(slug指定)
pub async fn get_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let lang = match params.lang {
        Some(ref l) => match l.as_str() {
            "ja" => Lang::Ja,
            "en" => Lang::En,
            _ => return StatusCode::BAD_REQUEST.into_response(),
        },
        None => Lang::Ja,
    };
    match state.db.find_page(&slug).await {
        Ok(Some(page)) => (StatusCode::OK, Json(to_page_detail_dto(page, &lang))).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ページ更新(slug指定)
pub async fn update_page(
    _claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(page_dto): Json<PageDTO>,
) -> impl IntoResponse {
    // 既存のページを取得してcreated_atを保持
    let existing_page = match state.db.find_page(&slug).await {
        Ok(Some(page)) => page,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let page = Page {
        id: existing_page.id,
        title: Name {
            ja: page_dto.title.ja,
            en: page_dto.title.en,
        },
        slug: page_dto.slug,
        content: Content {
            ja: page_dto.content.ja,
            en: page_dto.content.en,
        },
        created_at: existing_page.created_at,
        updated_at: DateTime::now(),
    };
    match state.db.update_page(&slug, &page).await {
        Ok(Some(updated_page)) => (StatusCode::OK, Json(updated_page)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ページ削除(slug指定)
pub async fn delete_page(
    _claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.db.delete_page(slug).await {
        Ok(res) => {
            if res.deleted_count >= 1 {
                StatusCode::NO_CONTENT.into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ページ一覧取得
pub async fn get_page_list(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.find_page_list().await {
        Ok(pages) => (StatusCode::OK, Json(to_page_list_dto(pages))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// 管理画面用のページ詳細データ取得
pub async fn get_admin_page_detail(
    _claims: Claims,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.db.find_page(&slug).await {
        Ok(Some(page)) => (StatusCode::OK, Json(to_admin_page_detail_dto(page))).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::Bson;

    #[test]
    fn create_public_postlist_filter_excludes_drafts() {
        let filter = create_public_postlist_filter(&None, &None, &None);
        assert_eq!(filter.get("is_draft"), Some(&Bson::Boolean(false)));
    }

    #[test]
    fn create_admin_postlist_filter_with_status_draft() {
        let status = Some(PostStatus::Draft);
        let filter = create_admin_postlist_filter(&None, &None, &None, &status);
        assert_eq!(filter.get("is_draft"), Some(&Bson::Boolean(true)));
    }

    #[test]
    fn create_admin_postlist_filter_with_status_published() {
        let status = Some(PostStatus::Published);
        let filter = create_admin_postlist_filter(&None, &None, &None, &status);
        assert_eq!(filter.get("is_draft"), Some(&Bson::Boolean(false)));
    }

    #[test]
    fn create_common_postlist_filter_sets_has_english_for_en() {
        let lang = Some("en".to_string());
        let filter = create_common_postlist_filter(&None, &None, &lang);
        assert_eq!(filter.get("has_english"), Some(&Bson::Boolean(true)));
    }
}
