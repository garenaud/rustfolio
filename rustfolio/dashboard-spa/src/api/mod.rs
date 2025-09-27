// dashboard-spa/src/api/mod.rs
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

/// Adapte si nécessaire (en dev: back = http://localhost:8080, sinon reverse-proxy => "")
const API_BASE: &str = "http://localhost:8080";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub about: String,
    pub website: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub date: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub percentage: u8,
    pub logo: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub category: String,
    pub repo_link: Option<String>,
    pub pdf_link: Option<String>,
    pub image: Option<String>,
    pub technologies: Vec<String>,
}

// -------- GET ----------

pub async fn get_profile() -> Result<Profile, String> {
    Request::get(&format!("{}/api/cv/profile", API_BASE))
        .send().await.map_err(to_err)?
        .json::<Profile>().await.map_err(to_err)
}

pub async fn get_experiences() -> Result<Vec<Experience>, String> {
    Request::get(&format!("{}/api/cv/experiences", API_BASE))
        .send().await.map_err(to_err)?
        .json::<Vec<Experience>>().await.map_err(to_err)
}

pub async fn get_skills() -> Result<Vec<Skill>, String> {
    Request::get(&format!("{}/api/cv/skills", API_BASE))
        .send().await.map_err(to_err)?
        .json::<Vec<Skill>>().await.map_err(to_err)
}

pub async fn get_projects() -> Result<Vec<Project>, String> {
    Request::get(&format!("{}/api/cv/projects", API_BASE))
        .send().await.map_err(to_err)?
        .json::<Vec<Project>>().await.map_err(to_err)
}

// -------- PUT (remplacement entier) ----------

pub async fn put_profile(payload: &Profile) -> Result<(), String> {
    Request::put(&format!("{}/api/cv/profile", API_BASE))
        .header("Content-Type", "application/json")
        .json(payload).map_err(to_err)?
        .send().await.map_err(to_err)?
        .ok().then_some(()).ok_or("PUT /profile failed".into())
}

pub async fn put_experiences(list: &[Experience]) -> Result<(), String> {
    Request::put(&format!("{}/api/cv/experiences", API_BASE))
        .header("Content-Type", "application/json")
        .json(list).map_err(to_err)?
        .send().await.map_err(to_err)?
        .ok().then_some(()).ok_or("PUT /experiences failed".into())
}

pub async fn put_skills(list: &[Skill]) -> Result<(), String> {
    Request::put(&format!("{}/api/cv/skills", API_BASE))
        .header("Content-Type", "application/json")
        .json(list).map_err(to_err)?
        .send().await.map_err(to_err)?
        .ok().then_some(()).ok_or("PUT /skills failed".into())
}

pub async fn put_projects(list: &[Project]) -> Result<(), String> {
    Request::put(&format!("{}/api/cv/projects", API_BASE))
        .header("Content-Type", "application/json")
        .json(list).map_err(to_err)?
        .send().await.map_err(to_err)?
        .ok().then_some(()).ok_or("PUT /projects failed".into())
}

fn to_err<E: std::fmt::Display>(e: E) -> String { e.to_string() }
