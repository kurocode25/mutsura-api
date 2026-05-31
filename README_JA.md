# Mutsura API

Mutsura APIは、Rustで構築された多言語対応のブログシステム向けAPIサーバーです。Webフレームワークに[Axum](https://github.com/tokio-rs/axum)、データベースに[MongoDB](https://www.mongodb.com)を使用しています。

> [!WARNING]
> このプロジェクトは現在開発中であり、仕様が大きく変更される可能性があります。また、セキュリティやパフォーマンスの検証が不十分なため、実用（本番環境での利用）には耐えられない旨をご理解の上、実験的な利用に留めてください。

## 主な機能

-   **認証**: JWT（JSON Web Token）およびリフレッシュトークンを使用した認証システム。
    -   ログイン (`/login`)
    -   アクセストークンのリフレッシュ (`/refresh`)
-   **ユーザー管理**: ユーザー情報の作成、取得、削除。
-   **記事管理 (多言語対応)**:
    -   日本語と英語のコンテンツを独立して管理。
    -   下書き機能、公開日管理。
    -   カテゴリやタグによるフィルタリング、ページネーション、全文検索機能。
-   **カテゴリ・タグ管理**: 記事を分類するための多言語対応カテゴリ・タグ管理。
-   **SSG サポート**: 静的サイト生成 (Static Site Generation) を高速化するための専用エンドポイント。

## セットアップと実行方法

### 1. 前提条件

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable) と Cargo。
-   [MongoDB](https://www.mongodb.com/try/download/community) サーバーが起動していること。

### 2. リポジトリのクローン

```bash
git clone <repository-url>
cd mutsura-api
```

### 3. 環境変数の設定

プロジェクトのルートディレクトリに `.env` ファイルを作成し、以下の内容を環境に合わせて記述します。

```env
MONGODB_URI=mongodb://localhost:27017
MONGODB_NAME=blog
JWT_SECRET=your_super_secret_key
CORS_ORIGIN=http://localhost:5173
PORT=127.0.0.1:3000
```

### 4. 管理者ユーザーの作成

初回実行時などに、管理者ユーザーを手動で作成する必要があります。

```bash
cargo run -- add-admin "日本語名" "English Name" "admin@example.com" "password123"
```

### 5. サーバーの起動

以下のコマンドでサーバーを起動します。

```bash
cargo run -- serve
```

デフォルトでは `http://127.0.0.1:3000` でリクエストを待機します。

## ライセンス

このプロジェクトは [MIT License](./LICENSE) の下で公開されています。
