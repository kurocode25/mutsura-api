# Mutsura API
Mutsura API is a multi-language blog system API server built with Rust. It uses the Axum web framework and MongoDB as its database.

> **Warning:** This project is currently in development and may undergo significant changes in specifications. Additionally, security and performance verification are insufficient, making it unsuitable for production use.

## Main Features
-   **Authentication**: JWT (JSON Web Token) and refresh token-based authentication system.
    -   Login (`/login`)
    -   Access token refresh (`/refresh`)
-   **User Management**: User information creation, retrieval, and deletion.
-   **Article Management (Multi-Language Support)**:
    -   Independent management of Japanese and English content.
    -   Draft function and publication date management.
    -   Filtering, pagination, and full-text search functions for categories and tags.
-   **Category/Tag Management**: Multi-language category/tag management for article classification.
-   **SSG (Static Site Generation) Support**: Dedicated endpoint for accelerating static site generation.

## Setup and Execution Method
### 1. Prerequisites
-   [Rust](https://www.rust-lang.org/tools/install) (latest stable) and Cargo.
-   A running MongoDB server.

### 2. Clone the Repository
```bash
git clone <repository-url>
cd mutsura-api
```

### 3. Environment Variable Setup
Create a `.env` file in the project root directory and fill it with the following content, adapting to your environment:
```env
MONGODB_URI=mongodb://localhost:27017
MONGODB_NAME=blog
JWT_SECRET=your_super_secret_key
CORS_ORIGIN=http://localhost:5173
PORT=127.0.0.1:3000
```

### 4. Creating an Admin User
Create a manual admin user for the first time or when necessary:
```bash
cargo run -- add-admin "Japanese Name" "English Name" "admin@example.com" "password123"
```

### 5. Server Startup
Start the server with the following command:
```bash
cargo run -- serve
```
By default, it listens for requests at `http://127.0.0.1:3000`.

## License
This project is published under the [MIT License](./LICENSE).
