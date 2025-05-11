use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use crate::routes::result::{ResponseBody, ResponseBodyTrait};
use crate::service::user::User;

pub fn get_auth_routes() -> Router {
    Router::new()
        .route("/", post(register))
}

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub password: String,
    pub name: String,
    pub desc: Option<String>,
    pub avatar: Option<String>,
}

pub async fn register(Json(payload): Json<RegisterDto>) -> ResponseBody {
    let user = User::register(
        &payload.password,
        payload.name,
        payload.desc,
        payload.avatar,
    )
    .await?;

    ResponseBody::success(user)
}