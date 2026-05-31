use crate::auth::RefreshToken;
use crate::model::{
    Category, CategoryWithCount, Page, Post, PostDetail, PostWithTotal, Tag, TagWithCount, User,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mockall::automock;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, Document},
    error::Result,
    Database,
};

#[automock]
#[async_trait]
pub trait Repository {
    // Userの取得
    async fn find_user(&self, user_id: &ObjectId) -> Result<Option<User>>;
    // Userのemailによる取得
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>>;
    // Userの作成
    async fn create_user(&self, user: User) -> Result<()>;
    // Userの更新
    async fn update_user(&self, user: User) -> Result<Option<User>>;
    // Userの削除
    async fn delete_user(&self, user_id: ObjectId) -> Result<DeleteResult>;
    // Postの作成
    async fn create_post(&self, post: Post) -> Result<()>;
    // 記事データの取得
    async fn find_post(&self, slug: &str) -> Result<Option<Post>>;
    // Post一覧の取得
    async fn find_posts(&self, query: Vec<Document>) -> Result<PostWithTotal>;
    // Post詳細の取得
    async fn find_post_detail(&self, slug: String) -> Result<Option<PostDetail>>;
    // 記事更新
    async fn update_post(&self, slug: &str, post: &Post) -> Result<Option<Post>>;
    // 記事削除
    async fn delete_post(&self, slug: String) -> Result<DeleteResult>;
    // カテゴリーの作成
    async fn create_caregory(&self, category: Category) -> Result<()>;
    // カテゴリーリストの取得
    async fn find_category_list(&self) -> Result<Vec<Category>>;
    // カテゴリーの取得
    async fn find_category(&self, slug: String) -> Result<Option<Category>>;
    // カテゴリーの更新
    async fn update_category(&self, slug: &str, category: &Category) -> Result<Option<Category>>;
    // カテゴリーの削除
    async fn delete_category(&self, slug: String) -> Result<DeleteResult>;
    // タグの作成
    async fn create_tag(&self, tag: &Tag) -> Result<InsertOneResult>;
    // 管理画面用のタグリストの取得
    async fn find_tag_list(&self) -> Result<Vec<Tag>>;
    // 公開記事用のタグリスト取得
    async fn find_tags_for_pub_post(&self, pileline: Vec<Document>) -> Result<Vec<Tag>>;
    // タグの取得
    async fn find_tag(&self, slug: String) -> Result<Option<Tag>>;
    // タグの更新
    async fn update_tag(&self, slug: &str, category: &Tag) -> Result<UpdateResult>;
    // タグの削除
    async fn delete_tag(&self, slug: String) -> Result<DeleteResult>;
    // リフレッシュトークンの登録
    async fn create_refresh_token(&self, token: RefreshToken) -> Result<()>;
    // リフレッシュトークンの取得
    async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    // リフレッシュトークンの無効化
    async fn revoke_refresh_token(&self, token: &str) -> Result<UpdateResult>;
    // SSG用 公開記事のslugリスト取得
    async fn find_ssg_post_slugs(&self, lang: &str) -> Result<Vec<String>>;
    // SSG用 公開記事数取得
    async fn find_ssg_post_count(&self, lang: &str) -> Result<i64>;
    // SSG用 タグのslugリストとタグを持つ公開記事の数取得
    async fn find_ssg_tag_list(&self, lang: &str) -> Result<Vec<TagWithCount>>;
    // SSG用 カテゴリーのslugリストとカテゴリーに属する公開記事の数取得
    async fn find_ssg_category_list(&self, lang: &str) -> Result<Vec<CategoryWithCount>>;
    // カテゴリーリストと記事数取得
    async fn find_category_list_with_count(&self) -> Result<Vec<CategoryWithCount>>;
    // ページの作成
    async fn create_page(&self, page: Page) -> Result<()>;
    // ページの一覧取得
    async fn find_page_list(&self) -> Result<Vec<Page>>;
    // ページの取得
    async fn find_page(&self, slug: &str) -> Result<Option<Page>>;
    // ページの更新
    async fn update_page(&self, slug: &str, page: &Page) -> Result<Option<Page>>;
    // ページの削除
    async fn delete_page(&self, slug: String) -> Result<DeleteResult>;
}

#[async_trait]
impl Repository for Database {
    // ユーザーの取得
    async fn find_user(&self, user_id: &ObjectId) -> Result<Option<User>> {
        let collection = self.collection::<User>("users");
        let filter = doc! { "_id": user_id };
        collection.find_one(filter).await
    }

    // emailをキーにユーザーを取得
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let collection = self.collection::<User>("users");
        let filter = doc! { "email": email };
        collection.find_one(filter).await
    }

    // ユーザー作成
    async fn create_user(&self, user: User) -> Result<()> {
        let collection = self.collection::<User>("users");
        collection.insert_one(user).await?;
        Ok(())
    }

    // ユーザー更新
    async fn update_user(&self, user: User) -> Result<Option<User>> {
        let collection = self.collection::<User>("users");
        // filterの作成
        let user_id = user.id.clone();
        let filter = doc! { "_id": user_id };

        // replace_oneの呼び出し
        collection.replace_one(filter, user).upsert(true).await?;
        let res = collection.find_one(doc! {"_id" : user_id}).await?;

        Ok(res)
    }

    // ユーザー削除
    async fn delete_user(&self, user_id: ObjectId) -> Result<DeleteResult> {
        let collection = self.collection::<User>("users");
        let res = collection.delete_one(doc! {"_id" : user_id}).await?;
        Ok(res)
    }

    // 記事作成
    async fn create_post(&self, post: Post) -> Result<()> {
        let collection = self.collection::<Post>("posts");
        collection.insert_one(post).await?;
        Ok(())
    }

    // 単純な記事取得(ユーザー情報などは含まない)
    async fn find_post(&self, slug: &str) -> Result<Option<Post>> {
        let collection = self.collection::<Post>("posts");
        let res = collection.find_one(doc! {"slug" : slug}).await?;
        Ok(res)
    }

    // 記事の合計値と記事一覧を取得
    async fn find_posts(&self, pipeline: Vec<Document>) -> Result<PostWithTotal> {
        let collection = self.collection::<Document>("posts");
        let doc = collection
            .aggregate(pipeline)
            .await?
            .try_next()
            .await?
            .unwrap_or(doc! {});
        let res = from_bson::<PostWithTotal>(mongodb::bson::Bson::Document(doc))?;
        Ok(res)
    }

    // 記事詳細を取得
    // Note: find_postとの違いは著書データを取得するか
    async fn find_post_detail(&self, slug: String) -> Result<Option<PostDetail>> {
        // 'posts' コレクションを取得
        let collection = self.collection::<Document>("posts");

        // スラッグに基づく検索フィルタ
        let mut filter = doc! {};
        filter.insert("slug", slug);
        // MongoDBのaggregationパイプラインを定義
        let pipeline: Vec<Document> = vec![
            doc! { "$match" : filter }, //{{{
            doc! { "$lookup" : {
                "from" : "users",              // 結合対象コレクションは"user"
                "localField" : "author_id",   // 'posts'コレクションの'author_id'
                "foreignField" : "_id",       // 'user'コレクションの_id
                "as" : "author"               // 結合結果は"author"というフィールドに格納
            }},
            doc! { "$unwind" : {
                "path" : "$author",
                "preserveNullAndEmptyArrays" : false
            }},
            doc! { "$project" : {
                "_id" : 1,
                "title" : 1,
                "slug" : 1,
                "is_draft" : 1,
                "has_english" : 1,
                "category" : 1,
                "tags" : 1,
                "content" : 1,
                "published_at" : 1,
                "updated_at" : 1,
                "created_at" : 1,
                "author" : {
                    "_id" : 1,
                    "name" : 1,
                    "email" : 1
                },
            }}, //}}}
        ];
        // パイプラインを実行して結果を取得
        let mut cursor = collection.aggregate(pipeline).await?;
        let doc = cursor.try_next().await?.unwrap_or(doc! {});

        // BSONドキュメントをPostDetail型に変換し、結果を返す
        let res = from_bson::<PostDetail>(mongodb::bson::Bson::Document(doc))?;
        Ok(Some(res))
    }

    // 記事更新
    async fn update_post(&self, slug: &str, post: &Post) -> Result<Option<Post>> {
        let collection = self.collection::<Post>("posts");

        // filterの作成
        let filter = doc! { "slug": slug };

        // replace_oneの呼び出し
        collection.replace_one(filter, post).upsert(true).await?;
        let res = self.find_post(&post.slug).await?;

        Ok(res)
    }

    // 記事削除
    async fn delete_post(&self, slug: String) -> Result<DeleteResult> {
        let collection = self.collection::<Post>("posts");
        let res = collection.delete_one(doc! {"slug" : slug}).await?;
        Ok(res)
    }

    // カテゴリーの追加
    async fn create_caregory(&self, category: Category) -> Result<()> {
        let collection = self.collection::<Category>("categories");
        collection.insert_one(category).await?;
        Ok(())
    }

    // カテゴリーリストの取得
    async fn find_category_list(&self) -> Result<Vec<Category>> {
        let collection = self.collection::<Category>("categories");
        let mut cursor = collection.find(doc! {}).await?;
        let mut res: Vec<Category> = vec![];
        while let Some(c) = cursor.try_next().await? {
            res.push(c);
        }
        Ok(res)
    }

    // カテゴリー詳細取得
    async fn find_category(&self, slug: String) -> Result<Option<Category>> {
        let collection = self.collection::<Category>("categories");
        collection.find_one(doc! {"slug" : slug}).await
    }

    // カテゴリーの更新
    // PUTメソッドでの使用を想定してアップサートの挙動とする
    async fn update_category(&self, slug: &str, category: &Category) -> Result<Option<Category>> {
        let collection = self.collection::<Category>("categories");

        // filterの作成
        let filter = doc! { "slug" : slug };

        // replace_oneの呼び出し
        collection
            .replace_one(filter, category)
            .upsert(true)
            .await?;
        let res = collection.find_one(doc! {"slug" : slug}).await?;

        Ok(res)
    }

    // カテゴリーの削除
    async fn delete_category(&self, slug: String) -> Result<DeleteResult> {
        let collection = self.collection::<Category>("categories");
        let res = collection.delete_one(doc! {"slug" : slug}).await?;
        Ok(res)
    }

    // タグの作成
    async fn create_tag(&self, tag: &Tag) -> Result<InsertOneResult> {
        let collection = self.collection::<Tag>("tags");
        let res = collection.insert_one(tag).await?;
        Ok(res)
    }

    // タグリストの取得
    async fn find_tag_list(&self) -> Result<Vec<Tag>> {
        let collection = self.collection::<Tag>("tags");
        let mut tags: Vec<Tag> = vec![];
        let mut cursor = collection.find(doc! {}).await?;
        while let Some(t) = cursor.try_next().await? {
            tags.push(t);
        }
        Ok(tags)
    }

    // 公開記事リスト用のタグリスト取得
    async fn find_tags_for_pub_post(&self, pipeline: Vec<Document>) -> Result<Vec<Tag>> {
        let collection = self.collection::<Document>("posts");
        let mut res: Vec<Tag> = Vec::new();
        let mut cursor = collection.aggregate(pipeline).await?;

        while let Some(result) = cursor.try_next().await? {
            res.push(bson::from_document(result)?);
        }

        Ok(res)
    }
    // タグの取得
    async fn find_tag(&self, slug: String) -> Result<Option<Tag>> {
        let collection = self.collection::<Tag>("tags");
        let res = collection.find_one(doc! {"slug": slug}).await?;
        Ok(res)
    }
    // タグの更新
    async fn update_tag(&self, slug: &str, category: &Tag) -> Result<UpdateResult> {
        let collection = self.collection::<Tag>("tags");

        // filterの作成
        let filter = doc! { "slug" : slug };

        // replace_oneの呼び出し
        let res = collection
            .replace_one(filter, category)
            .upsert(true)
            .await?;
        Ok(res)
    }
    // タグの削除
    async fn delete_tag(&self, slug: String) -> Result<DeleteResult> {
        let collection = self.collection::<Tag>("tags");
        let res = collection.delete_one(doc! {"slug" : slug}).await?;
        Ok(res)
    }

    // リフレッシュトークンの登録
    async fn create_refresh_token(&self, token: RefreshToken) -> Result<()> {
        let collection = self.collection::<RefreshToken>("refresh_tokens");
        collection.insert_one(token).await?;
        Ok(())
    }

    // リフレッシュトークンの取得
    async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let collection = self.collection::<RefreshToken>("refresh_tokens");
        let res = collection.find_one(doc! {"token": token}).await?;
        Ok(res)
    }

    // リフレッシュトークンの無効化
    async fn revoke_refresh_token(&self, token: &str) -> Result<UpdateResult> {
        let collection = self.collection::<RefreshToken>("refresh_tokens");
        let filter = doc! { "token": token };
        let update = doc! { "$set": { "revoked": true } };
        collection.update_one(filter, update).await
    }

    // SSG用 公開記事のslugリスト取得
    async fn find_ssg_post_slugs(&self, lang: &str) -> Result<Vec<String>> {
        let collection = self.collection::<Document>("posts");
        let mut filter = doc! {"is_draft": false};
        if lang == "en" {
            filter.insert("has_english", true);
        }
        let pipeline = vec![
            doc! {"$match": filter},
            doc! {"$project": {"_id": 0, "slug": 1}},
        ];
        let mut cursor = collection.aggregate(pipeline).await?;
        let mut slugs = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(slug) = from_bson::<String>(doc.get("slug").unwrap().clone()) {
                slugs.push(slug);
            }
        }
        Ok(slugs)
    }

    // SSG用 公開記事数取得
    async fn find_ssg_post_count(&self, lang: &str) -> Result<i64> {
        let collection = self.collection::<Document>("posts");
        let mut filter = doc! {"is_draft": false};
        if lang == "en" {
            filter.insert("has_english", true);
        }
        let pipeline = vec![doc! {"$match": filter}, doc! {"$count": "count"}];
        let mut cursor = collection.aggregate(pipeline).await?;
        let mut count = 0;
        if let Some(doc) = cursor.try_next().await? {
            if let Ok(c) = from_bson::<i64>(doc.get("count").unwrap().clone()) {
                count = c;
            }
        }
        Ok(count)
    }

    // SSG用 タグのslugリストとタグを持つ公開記事の数取得
    async fn find_ssg_tag_list(&self, lang: &str) -> Result<Vec<TagWithCount>> {
        let collection = self.collection::<Document>("posts");
        let mut filter = doc! {"is_draft": false};
        if lang == "en" {
            filter.insert("has_english", true);
        }
        let pipeline = vec![
            doc! {"$match": filter},
            doc! {"$unwind": "$tags"},
            doc! {"$group": {
                "_id": "$tags.slug",
                "name": {"$first": "$tags.name"},
                "count": {"$sum": 1}
            }},
            doc! {"$project": {
                "_id": 0,
                "slug": "$_id",
                "name": 1,
                "count": 1
            }},
        ];
        let mut cursor = collection.aggregate(pipeline).await?;
        let mut tags_with_count = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(tag_with_count) =
                from_bson::<TagWithCount>(mongodb::bson::Bson::Document(doc))
            {
                tags_with_count.push(tag_with_count);
            }
        }
        Ok(tags_with_count)
    }

    // SSG用 カテゴリーのslugリストとカテゴリーに属する公開記事の数取得
    async fn find_ssg_category_list(&self, lang: &str) -> Result<Vec<CategoryWithCount>> {
        let collection = self.collection::<Document>("posts");
        let mut filter = doc! {"is_draft": false};
        if lang == "en" {
            filter.insert("has_english", true);
        }
        let pipeline = vec![
            doc! {"$match": filter},
            doc! {"$group": {
                "_id": "$category.slug",
                "name": {"$first": "$category.name"},
                "count": {"$sum": 1}
            }},
            doc! {"$project": {
                "_id": 0,
                "slug": "$_id",
                "name": 1,
                "count": 1
            }},
        ];
        let mut cursor = collection.aggregate(pipeline).await?;
        let mut categories_with_count = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(category_with_count) =
                from_bson::<CategoryWithCount>(mongodb::bson::Bson::Document(doc))
            {
                categories_with_count.push(category_with_count);
            }
        }
        Ok(categories_with_count)
    }

    // カテゴリーリストと記事数取得
    async fn find_category_list_with_count(&self) -> Result<Vec<CategoryWithCount>> {
        let collection = self.collection::<Document>("posts");
        let pipeline = vec![
            doc! {"$match": {"is_draft": false}},
            doc! {"$group": {
                "_id": "$category.slug",
                "name": {"$first": "$category.name"},
                "count": {"$sum": 1}
            }},
            doc! {"$project": {
                "_id": 0,
                "slug": "$_id",
                "name": 1,
                "count": 1
            }},
        ];
        let mut cursor = collection.aggregate(pipeline).await?;
        let mut categories_with_count = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(category_with_count) =
                from_bson::<CategoryWithCount>(mongodb::bson::Bson::Document(doc))
            {
                categories_with_count.push(category_with_count);
            }
        }
        Ok(categories_with_count)
    }

    // ページの作成
    async fn create_page(&self, page: Page) -> Result<()> {
        let collection = self.collection::<Page>("pages");
        collection.insert_one(page).await?;
        Ok(())
    }

    // ページの一覧取得
    async fn find_page_list(&self) -> Result<Vec<Page>> {
        let collection = self.collection::<Page>("pages");
        let mut cursor = collection.find(doc! {}).await?;
        let mut pages: Vec<Page> = vec![];
        while let Some(page) = cursor.try_next().await? {
            pages.push(page);
        }
        Ok(pages)
    }

    // ページの取得
    async fn find_page(&self, slug: &str) -> Result<Option<Page>> {
        let collection = self.collection::<Page>("pages");
        collection.find_one(doc! {"slug" : slug}).await
    }

    // ページの更新
    async fn update_page(&self, slug: &str, page: &Page) -> Result<Option<Page>> {
        let collection = self.collection::<Page>("pages");
        let filter = doc! { "slug": slug };
        collection.replace_one(filter, page).upsert(true).await?;
        let res = self.find_page(&page.slug).await?;
        Ok(res)
    }

    // ページの削除
    async fn delete_page(&self, slug: String) -> Result<DeleteResult> {
        let collection = self.collection::<Page>("pages");
        let res = collection.delete_one(doc! {"slug" : slug}).await?;
        Ok(res)
    }
}
