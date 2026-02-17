use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    middleware::protected_route::protected_route,
    routes::{
        auth::{signin, signup},
        task::{add_task, delete_task, get_tasks, update_task},
    },
    state::AppState,
};

pub fn create_app(state: AppState) -> Router {
    info!("Building application router");

    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .merge(protected_routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

fn protected_routes() -> Router<AppState> {
    info!("Registering protected routes");

    Router::new()
        .route("/gettasks", get(get_tasks))
        .route("/addtask", post(add_task))
        .route("/updatetask/:id", put(update_task))
        .route("/deletetask/:id", delete(delete_task))
        .layer(middleware::from_fn(protected_route))
}
