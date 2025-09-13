use axum::{extract::State, http::StatusCode, response::Html};
use chrono::Datelike;
use crate::{state::AppState, templates::{IndexTemplate, ProjectsTpl, PortfolioTpl}};
use askama::Template;

pub async fn home(State(st): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let tpl = IndexTemplate {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        skills: &st.skills[..st.skills.len().min(5)],
        projects: &st.projects[..st.projects.len().min(3)],
    };
    tpl.render().map(Html).map_err(e500)
}

pub async fn projects_page(State(st): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let tpl = ProjectsTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        projects: &st.projects,
    };
    tpl.render().map(Html).map_err(e500)
}

pub async fn portfolio_page(State(_st): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let tpl = PortfolioTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
    };
    tpl.render().map(Html).map_err(e500)
}

fn e500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
