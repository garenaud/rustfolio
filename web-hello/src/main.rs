mod data;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router, serve,
};
use tokio::net::TcpListener;
use std::{fs, net::SocketAddr, sync::Arc};
use askama::Template;
use askama_axum::IntoResponse as _; // permet `tpl.into_response()`
use tower_http::services::ServeDir;
use chrono::Datelike;

// ---------- Templates SSR ----------

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    year: i32,
    name: &'a str,
    title: &'a str,
    tagline: &'a str,
    skills: &'a [data::Skill],
    projects: &'a [data::Project],
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTpl<'a> {
    year: i32,
    name: &'a str,
    title: &'a str,
    tagline: &'a str,
    projects: &'a [data::Project],
}

// ---------- State partagé ----------

#[derive(Clone)]
struct AppState {
    _experiences: Arc<Vec<data::Experience>>, // underscore = pas encore utilisé ici
    projects: Arc<Vec<data::Project>>,
    skills: Arc<Vec<data::Skill>>,
}

// ---------- API: filtres projets ----------

#[derive(serde::Deserialize, Debug)]
struct ProjectFilter {
    q: Option<String>,        // recherche dans title/description
    category: Option<String>, // match exact insensible à la casse
    tech: Option<String>,     // match exact sur une techno
    limit: Option<usize>,     // couper la liste
}

// ---------- Handlers ----------

async fn home(State(st): State<AppState>) -> impl IntoResponse {
    let tpl = IndexTemplate {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        skills: &st.skills[..st.skills.len().min(5)],
        projects: &st.projects[..st.projects.len().min(3)],
    };
    tpl.into_response()
}

async fn projects_page(State(st): State<AppState>) -> impl IntoResponse {
    ProjectsTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        projects: &st.projects,
    }
    .into_response() // matérialise une Response avant que `st` soit droppé (évite E0515)
}

#[derive(serde::Serialize)]
struct Info {
    status: &'static str,
    app: &'static str,
    version: &'static str,
}

async fn info_handler() -> Json<Info> {
    Json(Info {
        status: "ok",
        app: "web-hello",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn health() -> &'static str {
    "OK"
}

async fn api_projects(
    State(st): State<AppState>,
    Query(f): Query<ProjectFilter>,
) -> Json<Vec<data::Project>> {
    // nécessite `#[derive(Clone)]` sur data::Project (déjà ajouté côté data.rs)
    let mut out: Vec<data::Project> = st.projects.iter().cloned().collect();
    let norm = |s: &str| s.to_lowercase();

    if let Some(q) = &f.q {
        let qn = norm(q);
        out.retain(|p| norm(&p.title).contains(&qn) || norm(&p.description).contains(&qn));
    }
    if let Some(cat) = &f.category {
        let cn = norm(cat);
        out.retain(|p| norm(&p.category) == cn);
    }
    if let Some(tech) = &f.tech {
        let tn = norm(tech);
        out.retain(|p| p.technologies.iter().any(|t| norm(t) == tn));
    }
    if let Some(max) = f.limit {
        out.truncate(max);
    }

    Json(out)
}

// ---------- bootstrap ----------

#[tokio::main]
async fn main() {
    // charge les JSON (chemins relatifs au crate)
    let exp: Vec<data::Experience> = serde_json::from_str(
        &fs::read_to_string("data/experience_fr.json").expect("read exp"),
    )
    .expect("parse exp");

    let projects: Vec<data::Project> = serde_json::from_str(
        &fs::read_to_string("data/projects.json").expect("read projects"),
    )
    .expect("parse projects");

    let skills: Vec<data::Skill> =
        serde_json::from_str(&fs::read_to_string("data/skills.json").expect("read skills"))
            .expect("parse skills");

    let state = AppState {
        _experiences: Arc::new(exp),
        projects: Arc::new(projects),
        skills: Arc::new(skills),
    };

    let app = Router::new()
        // pages SSR
        .route("/", get(home))
        .route("/projects", get(projects_page))
        // API
        .route("/api/info", get(info_handler))
        .route("/api/projects", get(api_projects))
        // santé & statiques
        .route("/health", get(health))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
