use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;

use crate::state::AppState;

#[derive(Serialize, Deserialize, Default)]
pub struct CvPayload { #[serde(flatten)] pub data: Value }

#[derive(Serialize, Deserialize, Default)]
pub struct LayoutPayload { #[serde(flatten)] pub layout: Value }

fn uid() -> String { "1".into() } // MVP: user mock

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cv", get(get_cv).put(put_cv))
        .route("/layout", get(get_layout).put(put_layout))
}

async fn get_cv(State(st): State<AppState>) -> Json<CvPayload> {
    let user_id = uid();
    let rec: Option<(String,)> = sqlx::query_as("SELECT data FROM cv_data WHERE user_id=?")
        .bind(&user_id).fetch_optional(&st.db).await.unwrap();
    let data = rec
        .and_then(|(s,)| serde_json::from_str(&s).ok())
        .unwrap_or(Value::Object(Default::default()));
    Json(CvPayload { data })
}

async fn put_cv(State(st): State<AppState>, Json(p): Json<CvPayload>) -> Json<CvPayload> {
    let user_id = uid();
    let s = p.data.to_string();
    sqlx::query(r#"
        INSERT INTO cv_data (user_id, data, updated_at)
        VALUES (?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(user_id) DO UPDATE SET data=excluded.data, updated_at=CURRENT_TIMESTAMP
    "#).bind(&user_id).bind(&s).execute(&st.db).await.unwrap();
    Json(p)
}

async fn get_layout(State(st): State<AppState>) -> Json<LayoutPayload> {
    let user_id = uid();
    let rec: Option<(String,)> = sqlx::query_as("SELECT layout FROM cv_layout WHERE user_id=?")
        .bind(&user_id).fetch_optional(&st.db).await.unwrap();
    let layout = rec
        .and_then(|(s,)| serde_json::from_str(&s).ok())
        .unwrap_or(serde_json::json!({"rows":[]}));
    Json(LayoutPayload { layout })
}

async fn put_layout(State(st): State<AppState>, Json(p): Json<LayoutPayload>) -> Json<LayoutPayload> {
    let user_id = uid();
    let s = p.layout.to_string();
    sqlx::query(r#"
        INSERT INTO cv_layout (user_id, layout, updated_at)
        VALUES (?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(user_id) DO UPDATE SET layout=excluded.layout, updated_at=CURRENT_TIMESTAMP
    "#).bind(&user_id).bind(&s).execute(&st.db).await.unwrap();
    Json(p)
}
