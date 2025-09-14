use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Experience {
    pub date: String,
    pub r#type: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub category: String,
    pub technologies: Vec<String>,
    #[serde(rename = "repoLink")]
    pub repo_link: String,
    #[serde(rename = "pdfLink")]
    pub pdf_link: String,
    pub image: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub percentage: u8,
    pub logo: String,
    pub category: String,
}
