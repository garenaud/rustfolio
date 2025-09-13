use axum::{extract::{Query, State}, Json};
use crate::{state::AppState, data};

#[derive(serde::Serialize)]
pub struct Info {
    pub status: &'static str,
    pub app: &'static str,
    pub version: &'static str,
}

pub async fn info_handler() -> Json<Info> {
    Json(Info { status: "ok", app: "rustfolio", version: env!("CARGO_PKG_VERSION") })
}

#[derive(serde::Deserialize, Debug)]
pub struct ProjectFilter {
    pub q: Option<String>,
    pub category: Option<String>,
    pub tech: Option<String>,
    pub limit: Option<usize>,
}

pub async fn api_projects(
    State(st): State<AppState>,
    Query(f): Query<ProjectFilter>,
) -> Json<Vec<data::Project>> {
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
