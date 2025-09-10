use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Experience {
    pub date: String,
    pub r#type: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub category: String,
    pub technologies: Vec<String>,
    pub repoLink: String,
    pub pdfLink: String,
    pub image: String,
}

#[derive(Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub percentage: u8,
    pub logo: String,
    pub category: String,
}
