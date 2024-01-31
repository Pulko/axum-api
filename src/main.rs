mod api;

use api::{
    create_record, delete_record_by_id, retrieve_all_records, retrieve_record_by_id,
    update_record_by_id,
};
use axum::{routing::get, Router};
use sqlx::Executor;
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState { db };
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/users", get(retrieve_all_records).post(create_record))
        .route(
            "/users/:id",
            get(retrieve_record_by_id)
                .put(update_record_by_id)
                .delete(delete_record_by_id),
        )
        .with_state(state);

    Ok(router.into())
}
