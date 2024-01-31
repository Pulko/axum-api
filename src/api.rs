use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(sqlx::FromRow, serde::Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Deserialize)]
pub struct UserSubmission {
    name: String,
    age: i32,
}

#[derive(Deserialize)]
pub struct UpdateRecord {
    name: Option<String>,
    age: Option<i32>,
}

pub async fn retrieve_all_records(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, User>("SELECT * FROM USERS")
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(res))
}

pub async fn retrieve_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(res))
}

pub async fn create_record(
    State(state): State<AppState>,
    Json(json): Json<UserSubmission>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("INSERT INTO USERS (name, age) VALUES ($1, $2)")
        .bind(json.name)
        .bind(json.age)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn delete_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query_as::<_, User>("DELETE FROM USERS WHERE ID = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }
    Ok(StatusCode::OK)
}

pub async fn update_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<UserSubmission>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
        "UPDATE USERS (name, age) 
                SET name = (case when $1 is not null then $1 else name end),
                age = (case when $2 is not null then $2 else age end)
                WHERE 
                id = $3",
    )
    .bind(json.name)
    .bind(json.age)
    .bind(id)
    .execute(&state.db)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
