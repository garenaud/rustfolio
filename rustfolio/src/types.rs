use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub website: String,
    pub photo_url: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Experience {
    pub id: Option<i64>,
    pub date: String,
    #[serde(rename="type")]
    pub kind: String,          // "work" | "school" | ...
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Skill {
    pub id: Option<i64>,
    pub name: String,
    pub percentage: i64,
    pub logo: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Project {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub category: String,
    pub repo_link: String,
    pub pdf_link: String,
    pub image: String,
    pub technologies: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CvData {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
}
