use serde::{Deserialize, Serialize};
use yewdux::store::Store;
use gloo_net::http::Request;
use web_sys::console;

// ================== TYPES ==================
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

// ================== STORE ==================
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct CVStore {
    pub loaded: bool,
    pub profile: Option<Profile>,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub last_error: Option<String>,
    /// "api::<URL>" si succès, "demo" si fallback
    pub source: Option<String>,
}

impl CVStore {
    pub async fn fetch_all() -> Result<Self, String> {
        let bases = candidate_bases();

        let p  = get_json_multi::<Profile>(&bases, &["/api/cv/profile"]).await?;
        let e  = get_json_multi::<Vec<Experience>>(&bases, &["/api/cv/experiences"]).await?;
        let s  = get_json_multi::<Vec<Skill>>(&bases, &["/api/cv/skills"]).await?;
        let pr = get_json_multi::<Vec<Project>>(&bases, &["/api/cv/projects"]).await?;

        Ok(Self {
            loaded: true,
            profile: Some(p.data),
            experiences: e.data,
            skills: s.data,
            projects: pr.data,
            last_error: None,
            source: Some(format!("api::{}", pr.url)),
        })
    }

    pub fn load_demo(&mut self) {
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
        self.source = Some("demo".into());
    }
}

impl Store for CVStore {
    fn new(_ctx: &yewdux::Context) -> Self { Self::default() }
    fn should_notify(&self, old: &Self) -> bool { self != old }
}

// ============== HELPERS (avec URL de succès/erreur remontée) ==============

struct FetchOk<T> { data: T, url: String }

async fn get_json_multi<T: for<'de> serde::Deserialize<'de>>(bases: &[String], paths: &[&str]) -> Result<FetchOk<T>, String> {
    let mut last_err = String::new();
    for &path in paths {
        for base in bases {
            match try_get_json::<T>(base, path).await {
                Ok((data, url)) => return Ok(FetchOk { data, url }),
                Err(e) => {
                    last_err = e.clone();
                    console::warn_1(&format!("[CVStore] fail {}{} → {}", base, path, e).into());
                }
            }
        }
    }
    Err(last_err)
}

async fn try_get_json<T: for<'de> serde::Deserialize<'de>>(base: &str, path: &str) -> Result<(T, String), String> {
    let url = if base.is_empty() { path.to_string() } else { format!("{base}{path}") };
    console::log_1(&format!("[CVStore] GET {}", url).into());

    let resp = Request::get(&url)
        .send().await
        .map_err(|e| format!("network: {e} ({url})"))?;

    let status: u16 = resp.status();
    if !(200..=299).contains(&status) {
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("http {}: {} ({url})", status, truncate(&text, 200)));
    }

    let data = resp.json::<T>().await
        .map_err(|e| format!("json: {e} ({url})"))?;

    Ok((data, url))
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() > n { format!("{}…", &s[..n]) } else { s.to_string() }
}

fn candidate_bases() -> Vec<String> {
    vec![
        String::new(), 
        "http://localhost:8080".into(), 
        "http://127.0.0.1:8080".into(),
    ]
}
