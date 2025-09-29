use serde::{Deserialize, Serialize};
use yewdux::store::Store;
use gloo_net::http::Request;

/// Types alignés sur les endpoints JSON de l’API.
/// Adapte les noms de champs si besoin (ajoute #[serde(rename="...")] si ton backend diffère).

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
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

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Experience {
    pub date: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Skill {
    pub name: String,
    pub percentage: u8,
    pub logo: String,
    pub category: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub category: String,
    pub repo_link: String,
    pub pdf_link: String,
    pub image: String,
    pub technologies: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct CVStore {
    pub loaded: bool,
    pub profile: Option<Profile>,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub last_error: Option<String>,
}

impl CVStore {
    /// Charge tout depuis l’API (séquentiel pour éviter d’ajouter la dépendance `futures`)
    pub async fn fetch_all() -> Result<Self, String> {
        // NOTE: Si ton API est servie ailleurs, remplace par URL absolues:
        // "http://localhost:8080/api/cv/profile", etc.
        let profile: Profile = Request::get("/api/cv/profile")
            .send().await.map_err(err)?
            .json().await.map_err(err)?;
        let experiences: Vec<Experience> = Request::get("/api/cv/experiences")
            .send().await.map_err(err)?
            .json().await.map_err(err)?;
        let skills: Vec<Skill> = Request::get("/api/cv/skills")
            .send().await.map_err(err)?
            .json().await.map_err(err)?;
        let projects: Vec<Project> = Request::get("/api/cv/projects")
            .send().await.map_err(err)?
            .json().await.map_err(err)?;

        Ok(Self {
            loaded: true,
            profile: Some(profile),
            experiences,
            skills,
            projects,
            last_error: None,
        })
    }

    /// Fallback démo si l’API n’est pas joignable
    pub fn load_demo(&mut self) {
        if self.loaded { return; }
        self.profile = Some(Profile {
            first_name: "Jane".into(),
            last_name:  "Doe".into(),
            title: "Rust / Frontend".into(),
            email: "jane@doe.dev".into(),
            phone: "+41 00 000 00 00".into(),
            location: "Suisse".into(),
            about: "Demo user".into(),
            website: Some("https://janedoe.dev".into()),
            github: None, linkedin: None, twitter: None, picture: None,
        });
        self.experiences = vec![ Experience {
            date: "2023–…".into(), kind: "job".into(), title: "Rust Dev".into(),
            company: "Acme".into(), location: "CH".into(), tasks: vec!["Yew + Axum".into()]
        }];
        self.skills = vec![
            Skill { name:"Rust".into(), percentage:90, logo:"".into(), category:"Langages".into() },
            Skill { name:"Yew".into(),  percentage:70, logo:"".into(), category:"Frontend".into() },
        ];
        self.projects = vec![
            Project {
                title:"Rustfolio".into(), description:"CV Builder".into(), category:"web".into(),
                repo_link:"".into(), pdf_link:"".into(), image:"".into(), technologies: vec!["Rust".into(),"Yew".into()]
            }
        ];
        self.loaded = true;
    }
}

impl Store for CVStore {
    fn new(_ctx: &yewdux::Context) -> Self { Self::default() }
    fn should_notify(&self, old: &Self) -> bool { self != old }
}

fn err<E: std::fmt::Display>(e: E) -> String { e.to_string() }
