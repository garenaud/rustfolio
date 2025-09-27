use serde::{Deserialize, Serialize};
use serde_json::Value;
use yewdux::prelude::*;

// ---- Domain data ----
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Experience {
    pub date: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub percentage: u8,
    pub logo: String,
    pub category: String,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub category: String,
    #[serde(rename="repoLink")] pub repo_link: String,
    #[serde(rename="pdfLink")]  pub pdf_link:  String,
    pub image: String,
    pub technologies: Vec<String>,
}

// ---- Builder layout ----
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Layout { pub rows: Vec<Row> }
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Row { pub columns: Vec<Column> }
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Column { pub widgets: Vec<Widget> }

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Widget {
    Title { text: String, level: u8, #[serde(default)] bold: bool, #[serde(default)] align: Option<String> },
    ExperienceList { filter_type: Option<String> },
    SkillsGrid { category: Option<String> },
    ProjectCard { index: usize },
    Photo { url: String, rounded: bool },
}
impl Default for Widget {
    fn default() -> Self { Self::Title { text: "Titre".into(), level: 1, bold: false, align: None } }
}

// ---- App state & store ----
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct AppState {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub layout: Layout,

    // UI/transient
    pub selected_slot: Option<(usize, usize)>,
    pub selected_widget: Option<(usize, usize, usize)>,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug, Store)]
pub struct AppStore {
    pub state: AppState,
}

// ---- Helpers JSON (layout + data) ----
impl AppState {
    pub fn layout_from_json(&mut self, json: &Value) {
        if let Some(rows) = json.get("rows") {
            let mut new_layout = Layout { rows: vec![] };
            if let Some(rows_arr) = rows.as_array() {
                for r in rows_arr {
                    let mut row = Row { columns: vec![] };
                    if let Some(cols_arr) = r.get("columns").and_then(|c| c.as_array()) {
                        for c in cols_arr {
                            let mut col = Column { widgets: vec![] };
                            if let Some(widgets_arr) = c.get("widgets").and_then(|w| w.as_array()) {
                                for w in widgets_arr {
                                    if let Some(t) = w.get("type").and_then(|t| t.as_str()) {
                                        match t {
                                            "Title" => {
                                                let text = w.get("text").and_then(|v| v.as_str()).unwrap_or("Titre").to_string();
                                                let level = w.get("level").and_then(|v| v.as_u64()).unwrap_or(1) as u8;
                                                let bold = w.get("bold").and_then(|v| v.as_bool()).unwrap_or(false);
                                                let align = w.get("align").and_then(|v| v.as_str()).map(|s| s.to_string());
                                                col.widgets.push(Widget::Title { text, level, bold, align });
                                            }
                                            "ExperienceList" => {
                                                let filter_type = w.get("filter_type").and_then(|v| v.as_str()).map(|s| s.to_string());
                                                col.widgets.push(Widget::ExperienceList { filter_type });
                                            }
                                            "SkillsGrid" => {
                                                let category = w.get("category").and_then(|v| v.as_str()).map(|s| s.to_string());
                                                col.widgets.push(Widget::SkillsGrid { category });
                                            }
                                            "ProjectCard" => {
                                                let index = w.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                                col.widgets.push(Widget::ProjectCard { index });
                                            }
                                            "Photo" => {
                                                let url = w.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                                let rounded = w.get("rounded").and_then(|v| v.as_bool()).unwrap_or(true);
                                                col.widgets.push(Widget::Photo { url, rounded });
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            row.columns.push(col);
                        }
                    }
                    new_layout.rows.push(row);
                }
            }
            self.layout = new_layout;
        }
    }

    pub fn to_layout_json(&self) -> Value {
        let rows: Vec<Value> = self.layout.rows.iter().map(|row| {
            let columns: Vec<Value> = row.columns.iter().map(|col| {
                let widgets: Vec<Value> = col.widgets.iter().map(|w| {
                    match w {
                        Widget::Title { text, level, bold, align } => serde_json::json!({
                            "type": "Title", "text": text, "level": level, "bold": bold, "align": align
                        }),
                        Widget::ExperienceList { filter_type } => serde_json::json!({
                            "type": "ExperienceList", "filter_type": filter_type
                        }),
                        Widget::SkillsGrid { category } => serde_json::json!({
                            "type": "SkillsGrid", "category": category
                        }),
                        Widget::ProjectCard { index } => serde_json::json!({
                            "type": "ProjectCard", "index": index
                        }),
                        Widget::Photo { url, rounded } => serde_json::json!({
                            "type": "Photo", "url": url, "rounded": rounded
                        }),
                    }
                }).collect();
                serde_json::json!({ "widgets": widgets })
            }).collect();
            serde_json::json!({ "columns": columns })
        }).collect();

        serde_json::json!({ "rows": rows })
    }

    pub fn from_cv_json(&mut self, json: &serde_json::Value) {
        if let Some(p)  = json.get("profile")     { self.profile     = serde_json::from_value(p.clone()).unwrap_or_default(); }
        if let Some(e)  = json.get("experiences") { self.experiences = serde_json::from_value(e.clone()).unwrap_or_default(); }
        if let Some(s)  = json.get("skills")      { self.skills      = serde_json::from_value(s.clone()).unwrap_or_default(); }
        if let Some(pj) = json.get("projects")    { self.projects    = serde_json::from_value(pj.clone()).unwrap_or_default(); }
    }
    pub fn to_cv_json(&self) -> serde_json::Value {
        serde_json::json!({
            "profile": &self.profile,
            "experiences": &self.experiences,
            "skills": &self.skills,
            "projects": &self.projects
        })
    }
}
