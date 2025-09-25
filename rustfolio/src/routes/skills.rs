// src/routes/skills.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use crate::state::AppState;
use crate::types::{SkillIn, SkillOut};

type HandlerResult<T> = std::result::Result<T, (StatusCode, String)>;
fn ise<E: ToString>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cv/skills", get(list_skills).post(create_skill))
        .route("/cv/skills/:id", put(update_skill).delete(delete_skill))
        .route("/cv/skills/categories", get(list_skill_categories))
}

async fn list_skills(
    State(st): State<AppState>,
    auth: crate::auth::AuthUser,
) -> HandlerResult<Json<Vec<SkillOut>>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            id         AS "id!: i64",
            name       AS "name!: String",
            percentage AS "percentage?: i64",
            logo_url   AS "logo_url?: String",
            category   AS "category?: String"
        FROM skills
        WHERE user_id = ?
        ORDER BY updated_at DESC
        "#,
        auth.id
    )
    .fetch_all(&st.db)
    .await
    .map_err(ise)?;

    let out = rows
        .into_iter()
        .map(|r| SkillOut {
            id: r.id,
            name: r.name,
            percentage: r.percentage.and_then(|p| u8::try_from(p).ok()),
            logo_url: r.logo_url,
            category: r.category,
        })
        .collect();

    Ok(Json(out))
}

async fn create_skill(
    State(st): State<AppState>,
    auth: crate::auth::AuthUser,
    Json(s): Json<SkillIn>,
) -> HandlerResult<Json<SkillOut>> {
    let res = sqlx::query!(
        r#"
        INSERT INTO skills (user_id, name, percentage, logo_url, category, updated_at)
        VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        "#,
        auth.id,
        s.name,
        s.percentage.map(|p| i64::from(p)),
        s.logo_url,
        s.category
    )
    .execute(&st.db)
    .await
    .map_err(ise)?;

    let new_id = res.last_insert_rowid();

    let r = sqlx::query!(
        r#"
        SELECT
            id         AS "id!: i64",
            name       AS "name!: String",
            percentage AS "percentage?: i64",
            logo_url   AS "logo_url?: String",
            category   AS "category?: String"
        FROM skills
        WHERE id = ? AND user_id = ?
        "#,
        new_id,
        auth.id
    )
    .fetch_one(&st.db)
    .await
    .map_err(ise)?;

    Ok(Json(SkillOut {
        id: r.id,
        name: r.name,
        percentage: r.percentage.and_then(|p| u8::try_from(p).ok()),
        logo_url: r.logo_url,
        category: r.category,
    }))
}

async fn update_skill(
    State(st): State<AppState>,
    auth: crate::auth::AuthUser,
    Path(id): Path<i64>,
    Json(s): Json<SkillIn>,
) -> HandlerResult<Json<SkillOut>> {
    sqlx::query!(
        r#"
        UPDATE skills
        SET
            name        = ?,
            percentage  = ?,
            logo_url    = ?,
            category    = ?,
            updated_at  = CURRENT_TIMESTAMP
        WHERE id = ? AND user_id = ?
        "#,
        s.name,
        s.percentage.map(|p| i64::from(p)),
        s.logo_url,
        s.category,
        id,
        auth.id
    )
    .execute(&st.db)
    .await
    .map_err(ise)?;

    let r = sqlx::query!(
        r#"
        SELECT
            id         AS "id!: i64",
            name       AS "name!: String",
            percentage AS "percentage?: i64",
            logo_url   AS "logo_url?: String",
            category   AS "category?: String"
        FROM skills
        WHERE id = ? AND user_id = ?
        "#,
        id,
        auth.id
    )
    .fetch_one(&st.db)
    .await
    .map_err(ise)?;

    Ok(Json(SkillOut {
        id: r.id,
        name: r.name,
        percentage: r.percentage.and_then(|p| u8::try_from(p).ok()),
        logo_url: r.logo_url,
        category: r.category,
    }))
}

async fn delete_skill(
    State(st): State<AppState>,
    auth: crate::auth::AuthUser,
    Path(id): Path<i64>,
) -> HandlerResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM skills
        WHERE id = ? AND user_id = ?
        "#,
        id,
        auth.id
    )
    .execute(&st.db)
    .await
    .map_err(ise)?;

    Ok(())
}

async fn list_skill_categories(
    State(st): State<AppState>,
    auth: crate::auth::AuthUser,
) -> HandlerResult<Json<Vec<String>>> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT category AS "category!: String"
        FROM skills
        WHERE user_id = ? AND category IS NOT NULL
        ORDER BY category COLLATE NOCASE
        "#,
        auth.id
    )
    .fetch_all(&st.db)
    .await
    .map_err(ise)?;

    Ok(Json(rows.into_iter().map(|r| r.category).collect()))
}
