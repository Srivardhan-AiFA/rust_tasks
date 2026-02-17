use crate::{model::User, state::AppState};
use axum::{Json, extract::State};
use futures::stream::TryStreamExt;

pub async fn create_user(State(state): State<AppState>, Json(mut user): Json<User>) -> Json<User> {
    let collection = state.db.collection::<User>("users");

    let result = collection.insert_one(&user, None).await.unwrap();

    user.id = result.inserted_id.as_object_id();

    Json(user)
}

pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let collection = state.db.collection::<User>("users");

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut users = Vec::new();

    while let Some(user) = cursor.try_next().await.unwrap() {
        users.push(user);
    }

    Json(users)
}
