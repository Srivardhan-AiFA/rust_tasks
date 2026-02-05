use axum::Router;

use crate::routes;
use crate::state::Db;

pub fn create_app(db: Db) -> Router {
    routes::router(db)
}
