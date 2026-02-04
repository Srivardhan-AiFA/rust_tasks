use axum::{
    self, Router,
    response::{Html, IntoResponse},
    routing::get,
};

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // which calls one of these handlers
    async fn root() -> impl IntoResponse {
        Html("THE ROOOOOTTTTT")
    }
    async fn get_foo() -> impl IntoResponse {
        Html("GETTT FOOOO")
    }
    async fn post_foo() -> impl IntoResponse {
        Html("POSTTT FOOOO")
    }
    async fn foo_bar() -> impl IntoResponse {
        Html("FOOOO BARRR")
    }
}
