use axum::{extract::{State}, routing::{get, post}, Json, Router};
//use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::routes::auth::AuthUser;
use axum::http::StatusCode;

// Data shape stored as JSON blob (free-form for now)
#[derive(Serialize, Deserialize, Default)]
pub struct ProfileData {
    // --- Basic info
    pub full_name: Option<String>,
    pub title: Option<String>,
    pub tagline: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    // --- Summary
    pub summary: Option<String>,
    // --- Skills
    pub skills: Option<Vec<String>>,
    // --- Experience
    pub experiences: Option<Vec<ExperienceItem>>,
    // --- Education
    pub education: Option<Vec<EducationItem>>,
    // --- Projects
    pub projects: Option<Vec<ProjectItem>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExperienceItem {
    pub company: Option<String>,
    pub role: Option<String>,
    pub location: Option<String>,
    pub start: Option<String>, // ISO or free text
    pub end: Option<String>,
    pub bullets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EducationItem {
    pub school: Option<String>,
    pub degree: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub details: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tech: Option<Vec<String>>,
    pub link: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/profile", get(get_profile).post(save_profile))
        .route("/profile/export", get(export_profile))
        .route("/profile/import", post(import_profile))
}

// GET /api/profile -> returns JSON (empty default if none)
async fn get_profile(
    State(st): State<AppState>,
    user: AuthUser,
) -> Result<Json<ProfileData>, (StatusCode, String)> {
    let rec = sqlx::query!("SELECT data FROM user_profiles WHERE user_id = ?", user.id)
        .fetch_optional(&st.db).await
        .map_err(e500)?;
    if let Some(r) = rec {
        let data: ProfileData = serde_json::from_str(&r.data).unwrap_or_default();
        Ok(Json(data))
    } else {
        Ok(Json(ProfileData::default()))
    }
}

// POST /api/profile (JSON body)
async fn save_profile(
    State(st): State<AppState>,
    user: AuthUser,
    Json(data): Json<ProfileData>,
) -> Result<StatusCode, (StatusCode, String)> {
    let s = serde_json::to_string(&data).map_err(e500)?;
    sqlx::query!(
        r#"
        INSERT INTO user_profiles (user_id, data) VALUES (?, ?)
        ON CONFLICT(user_id) DO UPDATE SET data = excluded.data, updated_at = CURRENT_TIMESTAMP
        "#,
        user.id,
        s
    ).execute(&st.db).await.map_err(e500)?;
    Ok(StatusCode::NO_CONTENT)
}

// GET /api/profile/export -> download JSON
async fn export_profile(
    State(st): State<AppState>,
    user: AuthUser,
) -> Result<(axum::http::HeaderMap, String), (StatusCode, String)> {
    let rec = sqlx::query!("SELECT data FROM user_profiles WHERE user_id = ?", user.id)
        .fetch_optional(&st.db).await.map_err(e500)?;
    let json = rec.map(|r| r.data).unwrap_or_else(|| "{}".into());

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(axum::http::header::CONTENT_DISPOSITION, "attachment; filename=profile.json".parse().unwrap());
    Ok((headers, json))
}

// POST /api/profile/import (raw JSON in body)
async fn import_profile(
    State(st): State<AppState>,
    user: AuthUser,
    body: String,
) -> Result<StatusCode, (StatusCode, String)> {
    // Validate JSON minimally
    let _: serde_json::Value = serde_json::from_str(&body).map_err(e500)?;
    sqlx::query!(
        r#"
        INSERT INTO user_profiles (user_id, data) VALUES (?, ?)
        ON CONFLICT(user_id) DO UPDATE SET data = excluded.data, updated_at = CURRENT_TIMESTAMP
        "#,
        user.id,
        body
    ).execute(&st.db).await.map_err(e500)?;
    Ok(StatusCode::NO_CONTENT)
}

fn e500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}"))
}
