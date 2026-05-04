use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    sso_url: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct SampleRow {
    id: i32,
    name: String,
    value: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn get_row(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    headers: HeaderMap,
) -> Result<Json<SampleRow>, StatusCode> {
    // Extract bearer token
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify token via SSO
    let sso_resp: serde_json::Value = reqwest::Client::new()
        .post(format!("{}/verify", state.sso_url))
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if sso_resp.get("valid") != Some(&serde_json::Value::Bool(true)) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Fetch row from DB
    let row = sqlx::query_as::<_, SampleRow>("SELECT id, name, value FROM sample WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

#[tokio::main]
async fn main() {
    appbase::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/gsdemo".into());
    let sso_url = std::env::var("SSO_URL").unwrap_or_else(|_| "http://localhost:3001".into());

    let db = PgPool::connect(&database_url)
        .await
        .expect("DB connect failed");

    let state = AppState { db, sso_url };

    let app = Router::new()
        .route("/health", get(health))
        .route("/rows/:id", get(get_row))
        .with_state(state);

    let addr = std::env::var("CORE_ADDR").unwrap_or_else(|_| "0.0.0.0:3002".into());
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("core-server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
