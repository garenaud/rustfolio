use axum::{
    extract::{Path, State},
    routing::{delete, get, put, post},
    Json, Router,
};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::state::AppState;
use crate::types::{CvData, Experience, Profile, Project, Skill};

use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
type HandlerResult<T> = std::result::Result<T, (StatusCode, String)>;

mod skills;


/* =============================================================================
   DTOs & helpers
============================================================================= */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileDto {
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub title:      Option<String>,
    pub email:      Option<String>,
    pub phone:      Option<String>,
    pub address:    Option<String>,
    pub city:       Option<String>,
    pub country:    Option<String>,
    pub website:    Option<String>,
    pub photo_url:  Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub id:   i64,
    pub task: String,
}

fn uid() -> String {
    // à terme: récupérer l'user depuis la session
    "1".into()
}

// Trim simple côté serveur (évite d’introduire regex)
fn normalize_date_like(s: &str) -> String {
    s.trim().to_string()
}

/* =============================================================================
   Router
============================================================================= */

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cv/bulk", get(get_cv_bulk).put(put_cv_bulk))
        .route("/cv/profile", get(get_profile).put(put_profile))
        .route("/cv/experiences", get(list_experiences).post(create_experience))
        .route("/cv/experiences/:id", put(update_experience).delete(delete_experience))
        .route("/cv/experiences/:id/tasks", get(list_tasks).post(add_task))
        .route("/cv/experiences/:id/tasks/:task_id", delete(delete_task))
        .route("/cv/projects", get(list_projects).post(create_project))
        .route("/cv/projects/:id", put(update_project).delete(delete_project))
        .route("/cv/projects/:id/tech", get(list_project_tech).post(add_project_tech))
        .route("/cv/projects/:id/tech/:tech_id", delete(delete_project_tech))
}


/* =============================================================================
   BULK
============================================================================= */

async fn get_cv_bulk(State(st): State<AppState>) -> Json<CvData> {
    let user_id = uid();
    let profile = get_profile_inner(&st.db, &user_id).await.unwrap_or_default();
    let experiences = list_experiences_inner(&st.db, &user_id).await.unwrap_or_default();
    let skills = list_skills_inner(&st.db, &user_id).await.unwrap_or_default();
    let projects = list_projects_inner(&st.db, &user_id).await.unwrap_or_default();
    Json(CvData { profile, experiences, skills, projects })
}

async fn put_cv_bulk(State(st): State<AppState>, Json(cv): Json<CvData>) -> Json<serde_json::Value> {
    let user_id = uid();
    // pragmatique: pas de transaction pour l’instant
    put_profile_inner(&st.db, &user_id, &cv.profile).await.unwrap();
    replace_experiences_inner(&st.db, &user_id, &cv.experiences).await.unwrap();
    replace_skills_inner(&st.db, &user_id, &cv.skills).await.unwrap();
    replace_projects_inner(&st.db, &user_id, &cv.projects).await.unwrap();
    Json(json!({ "ok": true }))
}

/* =============================================================================
   PROFILE
============================================================================= */

async fn get_profile(State(st): State<AppState>) -> Json<Profile> {
    let user_id = uid();
    Json(get_profile_inner(&st.db, &user_id).await.unwrap_or_default())
}

async fn put_profile(
    State(st): State<AppState>,
    Json(patch): Json<ProfileDto>
) -> Json<serde_json::Value> {
    let user_id = uid();

    let current = get_profile_inner(&st.db, &user_id).await.unwrap_or_default();
    let merged = merge_profile(current, patch);

    put_profile_inner(&st.db, &user_id, &merged).await.unwrap();
    Json(json!({ "ok": true }))
}

fn merge_profile(curr: Profile, patch: ProfileDto) -> Profile {
    Profile {
        first_name: patch.first_name.unwrap_or(curr.first_name),
        last_name:  patch.last_name.unwrap_or(curr.last_name),
        title:      patch.title.unwrap_or(curr.title),
        email:      patch.email.unwrap_or(curr.email),
        phone:      patch.phone.unwrap_or(curr.phone),
        address:    patch.address.unwrap_or(curr.address),
        city:       patch.city.unwrap_or(curr.city),
        country:    patch.country.unwrap_or(curr.country),
        website:    patch.website.unwrap_or(curr.website),
        photo_url:  patch.photo_url.unwrap_or(curr.photo_url),
    }
}

async fn get_profile_inner(db: &Pool<Sqlite>, user_id: &str) -> sqlx::Result<Profile> {
    let rec = sqlx::query!(
        r#"
        SELECT first_name, last_name, title, email, phone,
               address, city, country, website, photo_url
        FROM profiles WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_optional(db)
    .await?;

    if let Some(r) = rec {
        Ok(Profile {
            first_name: r.first_name.unwrap_or_default(),
            last_name: r.last_name.unwrap_or_default(),
            title: r.title.unwrap_or_default(),
            email: r.email.unwrap_or_default(),
            phone: r.phone.unwrap_or_default(),
            address: r.address.unwrap_or_default(),
            city: r.city.unwrap_or_default(),
            country: r.country.unwrap_or_default(),
            website: r.website.unwrap_or_default(),
            photo_url: r.photo_url.unwrap_or_default(),
        })
    } else {
        Ok(Profile::default())
    }
}

async fn put_profile_inner(db: &Pool<Sqlite>, user_id: &str, p: &Profile) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO profiles
          (user_id, first_name, last_name, title, email, phone,
           address, city, country, website, photo_url, updated_at)
        VALUES
          (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(user_id) DO UPDATE SET
          first_name = excluded.first_name,
          last_name  = excluded.last_name,
          title      = excluded.title,
          email      = excluded.email,
          phone      = excluded.phone,
          address    = excluded.address,
          city       = excluded.city,
          country    = excluded.country,
          website    = excluded.website,
          photo_url  = excluded.photo_url,
          updated_at = CURRENT_TIMESTAMP
        "#,
        user_id,
        p.first_name,
        p.last_name,
        p.title,
        p.email,
        p.phone,
        p.address,
        p.city,
        p.country,
        p.website,
        p.photo_url
    )
    .execute(db)
    .await?;
    Ok(())
}

/* =============================================================================
   EXPERIENCES + TASKS
============================================================================= */

async fn list_experiences(State(st): State<AppState>) -> Json<Vec<Experience>> {
    let user_id = uid();
    Json(list_experiences_inner(&st.db, &user_id).await.unwrap_or_default())
}

async fn create_experience(State(st): State<AppState>, Json(mut e): Json<Experience>) -> Json<Experience> {
    let user_id = uid();

    // normalise
    e.date_start = normalize_date_like(&e.date_start);
    e.date_end   = normalize_date_like(&e.date_end);

    let date_start = normalize_date_like(&e.date_start);
    let date_end = normalize_date_like(&e.date_end);
    let id = sqlx::query!(
        r#"
        INSERT INTO experiences
          (user_id, date_start, date_end, kind, title, company, location, website, updated_at)
        VALUES
          (?,?,?,?,?,?,?,?,CURRENT_TIMESTAMP)
        "#,
        user_id,
        date_start,
        date_end,
        e.kind,
        e.title,
        e.company,
        e.location,
        e.website
    )
    .execute(&st.db)
    .await
    .unwrap()
    .last_insert_rowid();

    e.id = Some(id);
    // tasks seront ajoutées via l’endpoint dédié
    e.tasks = vec![];
    Json(e)
}

async fn update_experience(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Json(mut e): Json<Experience>,
) -> Json<serde_json::Value> {
    // normalise
    e.date_start = normalize_date_like(&e.date_start);
    e.date_end   = normalize_date_like(&e.date_end);

    sqlx::query!(
        r#"
        UPDATE experiences
           SET date_start = ?,
               date_end   = ?,
               kind       = ?,
               title      = ?,
               company    = ?,
               location   = ?,
               website    = ?,
               updated_at = CURRENT_TIMESTAMP
         WHERE id = ?
        "#,
        e.date_start,
        e.date_end,
        e.kind,
        e.title,
        e.company,
        e.location,
        e.website,
        id
    )
    .execute(&st.db)
    .await
    .unwrap();

    Json(json!({ "ok": true }))
}

async fn delete_experience(State(st): State<AppState>, Path(id): Path<i64>) -> Json<serde_json::Value> {
    // d’abord delete les tasks enfants (FK)
    sqlx::query!("DELETE FROM experience_tasks WHERE experience_id = ?", id)
        .execute(&st.db)
        .await
        .unwrap();

    sqlx::query!("DELETE FROM experiences WHERE id = ?", id)
        .execute(&st.db)
        .await
        .unwrap();

    Json(json!({ "ok": true }))
}

// Liste des tasks (avec id) pour une expérience
async fn list_tasks(State(st): State<AppState>, Path(id): Path<i64>) -> Json<Vec<TaskItem>> {
    let rows = sqlx::query!(
        r#"SELECT id as "id!: i64", task FROM experience_tasks WHERE experience_id = ? ORDER BY id"#,
        id
    )
    .fetch_all(&st.db)
    .await
    .unwrap();

    Json(rows.into_iter().map(|r| TaskItem { id: r.id, task: r.task }).collect())
}

// Ajout d’une task: renvoie le TaskItem créé (id + task)
async fn add_task(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Json<TaskItem> {
    let task = body.get("task").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();

    let new_id = sqlx::query!(
        "INSERT INTO experience_tasks (experience_id, task) VALUES (?, ?)",
        id,
        task
    )
    .execute(&st.db)
    .await
    .unwrap()
    .last_insert_rowid();

    Json(TaskItem { id: new_id, task })
}

async fn delete_task(
    State(st): State<AppState>,
    Path((exp_id, task_id)): Path<(i64, i64)>,
) -> Json<serde_json::Value> {
    sqlx::query!(
        "DELETE FROM experience_tasks WHERE experience_id = ? AND id = ?",
        exp_id,
        task_id
    )
    .execute(&st.db)
    .await
    .unwrap();

    Json(json!({ "ok": true }))
}

async fn list_experiences_inner(db: &Pool<Sqlite>, user_id: &str) -> sqlx::Result<Vec<Experience>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            id as "id!: i64",
            date_start,
            date_end,
            kind,
            title,
            company,
            location,
            website
        FROM experiences
        WHERE user_id = ?
        ORDER BY id
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        // pour le bulk, on ne renvoie que les tasks en Vec<String> (pas les IDs)
        let tasks_rows = sqlx::query!(
            "SELECT task FROM experience_tasks WHERE experience_id = ? ORDER BY id",
            r.id
        )
        .fetch_all(db)
        .await?;

        out.push(Experience {
            id:         Some(r.id),
            date_start: r.date_start,
            date_end:   r.date_end,
            kind:       r.kind.unwrap_or_default(),
            title:      r.title.unwrap_or_default(),
            company:    r.company.unwrap_or_default(),
            location:   r.location.unwrap_or_default(),
            website:    r.website,
            tasks:      tasks_rows.into_iter().map(|t| t.task).collect(),
        });
    }
    Ok(out)
}

async fn replace_experiences_inner(db: &Pool<Sqlite>, user_id: &str, list: &[Experience]) -> sqlx::Result<()> {
    // purge tasks pour les experiences de l’utilisateur
    sqlx::query!(
        "DELETE FROM experience_tasks WHERE experience_id IN (SELECT id FROM experiences WHERE user_id = ?)",
        user_id
    )
    .execute(db)
    .await?;

    // purge experiences
    sqlx::query!("DELETE FROM experiences WHERE user_id = ?", user_id)
        .execute(db)
        .await?;

    // réinsert
    for e in list {
        let date_start = normalize_date_like(&e.date_start);
        let date_end = normalize_date_like(&e.date_end);

        let id = sqlx::query!(
            r#"
            INSERT INTO experiences
            (user_id, date_start, date_end, kind, title, company, location, website, updated_at)
            VALUES
            (?,?,?,?,?,?,?,?,CURRENT_TIMESTAMP)
            "#,
            user_id,
            date_start,
            date_end,
            e.kind,
            e.title,
            e.company,
            e.location,
            e.website
        )
        .execute(db)
        .await?
        .last_insert_rowid();

        for t in &e.tasks {
            sqlx::query!(
                "INSERT INTO experience_tasks (experience_id, task) VALUES (?, ?)",
                id,
                t
            )
            .execute(db)
            .await?;
        }
    }
    Ok(())
}


async fn list_skills_inner(db: &Pool<Sqlite>, user_id: &str) -> sqlx::Result<Vec<Skill>> {
    let rows = sqlx::query!(
        r#"
        SELECT id as "id!: i64", name, percentage, logo, category
          FROM skills
         WHERE user_id = ?
         ORDER BY id
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Skill {
            id: Some(r.id),
            name: r.name, // String direct
            percentage: (r.percentage as i32),
            logo: r.logo.unwrap_or_default(),
            category: r.category.unwrap_or_default(),
        })
        .collect())
}

async fn replace_skills_inner(db: &Pool<Sqlite>, user_id: &str, list: &[Skill]) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM skills WHERE user_id = ?", user_id)
        .execute(db)
        .await?;

    for sk in list {
        let perc_i64 = sk.percentage as i64;
        sqlx::query!(
            r#"
            INSERT INTO skills
              (user_id, name, percentage, logo, category, updated_at)
            VALUES
              (?,?,?,?,?,CURRENT_TIMESTAMP)
            "#,
            user_id,
            sk.name,
            perc_i64,
            sk.logo,
            sk.category
        )
        .execute(db)
        .await?;
    }
    Ok(())
}

/* =============================================================================
   PROJECTS + TECHNOLOGIES
============================================================================= */

async fn list_projects(State(st): State<AppState>) -> Json<Vec<Project>> {
    let user_id = uid();
    Json(list_projects_inner(&st.db, &user_id).await.unwrap_or_default())
}

async fn create_project(State(st): State<AppState>, Json(mut p): Json<Project>) -> Json<Project> {
    let user_id = uid();
    let id = sqlx::query!(
        r#"
        INSERT INTO projects
          (user_id, title, description, category, repo_link, pdf_link, image, updated_at)
        VALUES
          (?,?,?,?,?,?,?,CURRENT_TIMESTAMP)
        "#,
        user_id,
        p.title,
        p.description,
        p.category,
        p.repo_link,
        p.pdf_link,
        p.image
    )
    .execute(&st.db)
    .await
    .unwrap()
    .last_insert_rowid();

    p.id = Some(id);
    Json(p)
}

async fn update_project(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Json(p): Json<Project>,
) -> Json<serde_json::Value> {
    // update projet
    sqlx::query!(
        r#"
        UPDATE projects
           SET title = ?,
               description = ?,
               category = ?,
               repo_link = ?,
               pdf_link = ?,
               image = ?,
               updated_at = CURRENT_TIMESTAMP
         WHERE id = ?
        "#,
        p.title,
        p.description,
        p.category,
        p.repo_link,
        p.pdf_link,
        p.image,
        id
    )
    .execute(&st.db)
    .await
    .unwrap();

    // refresh ses technologies
    sqlx::query!("DELETE FROM project_technologies WHERE project_id = ?", id)
        .execute(&st.db)
        .await
        .unwrap();

    for t in &p.technologies {
        sqlx::query!(
            "INSERT INTO project_technologies (project_id, tech) VALUES (?, ?)",
            id,
            t
        )
        .execute(&st.db)
        .await
        .unwrap();
    }

    Json(json!({ "ok": true }))
}

async fn delete_project(State(st): State<AppState>, Path(id): Path<i64>) -> Json<serde_json::Value> {
    sqlx::query!("DELETE FROM project_technologies WHERE project_id = ?", id)
        .execute(&st.db)
        .await
        .unwrap();

    sqlx::query!("DELETE FROM projects WHERE id = ?", id)
        .execute(&st.db)
        .await
        .unwrap();

    Json(json!({ "ok": true }))
}

async fn list_project_tech(State(st): State<AppState>, Path(id): Path<i64>) -> Json<Vec<String>> {
    let rows = sqlx::query!(
        r#"SELECT tech FROM project_technologies WHERE project_id = ? ORDER BY id"#,
        id
    )
    .fetch_all(&st.db)
    .await
    .unwrap();

    Json(rows.into_iter().map(|r| r.tech).collect())
}

async fn add_project_tech(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let tech = body.get("tech").and_then(|v| v.as_str()).unwrap_or("").to_string();

    sqlx::query!(
        "INSERT INTO project_technologies (project_id, tech) VALUES (?, ?)",
        id,
        tech
    )
    .execute(&st.db)
    .await
    .unwrap();

    Json(json!({ "ok": true }))
}

async fn delete_project_tech(
    State(st): State<AppState>,
    Path((id, tech_id)): Path<(i64, i64)>,
) -> Json<serde_json::Value> {
    sqlx::query!(
        "DELETE FROM project_technologies WHERE project_id = ? AND id = ?",
        id,
        tech_id
    )
    .execute(&st.db)
    .await
    .unwrap();

    Json(json!({ "ok": true }))
}

async fn list_projects_inner(db: &Pool<Sqlite>, user_id: &str) -> sqlx::Result<Vec<Project>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            id as "id!: i64",
            title, description, category, repo_link, pdf_link, image
        FROM projects
        WHERE user_id = ?
        ORDER BY id
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let tech_rows = sqlx::query!(
            r#"SELECT tech FROM project_technologies WHERE project_id = ? ORDER BY id"#,
            r.id
        )
        .fetch_all(db)
        .await?;

        out.push(Project {
            id: Some(r.id),
            title: r.title,
            description: r.description.unwrap_or_default(),
            category: r.category.unwrap_or_default(),
            repo_link: r.repo_link.unwrap_or_default(),
            pdf_link: r.pdf_link.unwrap_or_default(),
            image: r.image.unwrap_or_default(),
            technologies: tech_rows.into_iter().map(|t| t.tech).collect(),
        });
    }
    Ok(out)
}

async fn replace_projects_inner(db: &Pool<Sqlite>, user_id: &str, list: &[Project]) -> sqlx::Result<()> {
    // purge enfants d’abord
    let pids = sqlx::query!(
        r#"SELECT id as "id!: i64" FROM projects WHERE user_id = ?"#,
        user_id
    )
    .fetch_all(db)
    .await?;

    for r in pids {
        sqlx::query!("DELETE FROM project_technologies WHERE project_id = ?", r.id)
            .execute(db)
            .await?;
    }

    sqlx::query!("DELETE FROM projects WHERE user_id = ?", user_id)
        .execute(db)
        .await?;

    for p in list {
        let id = sqlx::query!(
            r#"
            INSERT INTO projects
              (user_id, title, description, category, repo_link, pdf_link, image, updated_at)
            VALUES
              (?,?,?,?,?,?,?,CURRENT_TIMESTAMP)
            "#,
            user_id,
            p.title,
            p.description,
            p.category,
            p.repo_link,
            p.pdf_link,
            p.image
        )
        .execute(db)
        .await?
        .last_insert_rowid();

        for t in &p.technologies {
            sqlx::query!(
                "INSERT INTO project_technologies (project_id, tech) VALUES (?, ?)",
                id,
                t
            )
            .execute(db)
            .await?;
        }
    }
    Ok(())
}

pub fn mount_skills_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/cv/skills", get(list_skills).post(create_skill))
        .with_state(state.clone())
        .route("/api/cv/skills/:id", put(update_skill).delete(delete_skill))
        .route("/api/cv/skills/categories", get(list_skill_categories))
        .with_state(state)
}

async fn list_skills(
    State(db): State<&sqlx::SqlitePool>,
    auth: crate::auth::AuthUser,
) -> HandlerResult<Json<Vec<crate::types::SkillOut>>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, percentage, logo_url, category
        FROM skills
        WHERE user_id = ?
        ORDER BY id DESC
        "#,
        auth.id
    )
    .fetch_all(db)
    .await?;

    let out = rows
        .into_iter()
        .map(|r| crate::types::SkillOut {
            id: r.id.unwrap_or_default(),
            name: r.name,
            // percentage est Option<i64> si SQLite → map vers Option<u8>
            percentage: u8::try_from(r.percentage).ok(),
            logo_url: r.logo_url,
            category: r.category.unwrap_or_default(),
        })
        .collect();

    Ok(Json(out))
}

async fn create_skill(
    State(db): State<&sqlx::SqlitePool>,
    auth: crate::auth::AuthUser,
    Json(s): Json<crate::types::SkillIn>,
) -> HandlerResult<Json<crate::types::SkillOut>> {
    let id = sqlx::query!(
        r#"
        INSERT INTO skills (user_id, name, percentage, logo_url, category, updated_at)
        VALUES (?,?,?,?,?, CURRENT_TIMESTAMP)
        "#,
        auth.id,
        s.name,
        s.percentage.map(|p| p as i64),
        s.logo_url,
        s.category
    )
    .execute(db)
    .await?
    .last_insert_rowid();

    Ok(Json(crate::types::SkillOut {
        id,
        name: s.name,
        percentage: s.percentage,
        logo_url: s.logo_url,
        category: s.category,
    }))
}

async fn update_skill(
    State(db): State<&sqlx::SqlitePool>,
    auth: crate::auth::AuthUser,
    Path(id): Path<i64>,
    Json(s): Json<crate::types::SkillIn>,
) -> Result<Json<crate::types::SkillOut>, (StatusCode, String)> {
    // sécurité: ne mettre à jour que la ligne de l'utilisateur
    sqlx::query!(
        r#"
        UPDATE skills
        SET name = ?, percentage = ?, logo_url = ?, category = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND user_id = ?
        "#,
        s.name,
        s.percentage.map(|p| p as i64),
        s.logo_url,
        s.category,
        id,
        auth.id
    )
    .execute(db)
    .await?;

    Ok(Json(crate::types::SkillOut {
        id,
        name: s.name,
        percentage: s.percentage,
        logo_url: s.logo_url,
        category: s.category,
    }))
}

async fn delete_skill(
    State(db): State<&sqlx::SqlitePool>,
    auth: crate::auth::AuthUser,
    Path(id): Path<i64>,
) -> HandlerResult<()>  {
    sqlx::query!(
        r#"
        DELETE FROM skills
        WHERE id = ? AND user_id = ?
        "#,
        id,
        auth.id
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn list_skill_categories(
    State(db): State<&sqlx::SqlitePool>,
    auth: crate::auth::AuthUser,
) -> HandlerResult<Json<Vec<String>>> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT category
        FROM skills
        WHERE user_id = ?
        ORDER BY category COLLATE NOCASE
        "#,
        auth.id
    )
    .fetch_all(db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    //.await?;

    Ok(Json(rows.into_iter().map(|r| r.category).collect()))
}
