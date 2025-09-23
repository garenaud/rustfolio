use serde::{Deserialize, Serialize};

/* =================== PROFILE =================== */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Profile {
    pub first_name: String,
    pub last_name:  String,
    pub title:      String,
    pub email:      String,
    pub phone:      String,
    pub address:    String,
    pub city:       String,
    pub country:    String,
    pub website:    String,
    pub photo_url:  String,
}

/* =================== EXPERIENCE =================== */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Experience {
    pub id: Option<i64>,
    pub date_start: String,
    pub date_end: String,
    pub kind: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub website: String,
    pub tasks: Vec<String>,
}


/* =================== SKILL =================== */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Skill {
    pub id:         Option<i64>,
    pub name:       String,
    pub percentage: i32,
    pub logo:       String,
    pub category:   String,
}

/* =================== PROJECT =================== */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Project {
    pub id:          Option<i64>,
    pub title:       String,
    pub description: String,
    pub category:    String,
    pub repo_link:   String,
    pub pdf_link:    String,
    pub image:       String,
    pub technologies: Vec<String>,
}

/* =================== CV BULK =================== */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CvData {
    pub profile:     Profile,
    pub experiences: Vec<Experience>,
    pub skills:      Vec<Skill>,
    pub projects:    Vec<Project>,
}
