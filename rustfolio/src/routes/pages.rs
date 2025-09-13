// src/routes/pages.rs
use askama::Template;              // <- pour avoir .render()
use askama_axum::IntoResponse;     // <- pour home() si tu retournes un Template
use chrono::Datelike;

use crate::templates::{HomeTpl, ProjectsTpl, PortfolioTpl};
use crate::state::AppState;


#[derive(Template)]
#[template(source = "<h1>OK inline {{ year }}</h1>", ext = "html")]
pub struct InlineTpl {
    pub year: i32,
}

pub async fn debug_inline() -> InlineTpl {
    InlineTpl { year: chrono::Utc::now().year() }
}

// Pas d'emprunt -> on peut retourner directement le Template
pub async fn home() -> HomeTpl {
    HomeTpl { year: chrono::Utc::now().year() }
}

// Emprunts -> on rend en String puis Html<String>
pub async fn projects_page(
    axum::extract::State(st): axum::extract::State<AppState>,
) -> axum::response::Html<String> {
    let html = ProjectsTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        projects: &st.projects, // <- emprunt
    }
    .render()
    .expect("Askama render projects.html");

    axum::response::Html(html)
}

pub async fn portfolio_page(
    axum::extract::State(_st): axum::extract::State<AppState>,
) -> axum::response::Html<String> {
    let html = PortfolioTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
    }
    .render()
    .expect("Askama render portfolio.html");

    axum::response::Html(html)
}
