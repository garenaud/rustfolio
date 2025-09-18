use serde::{Deserialize, Serialize};
use serde_json::Value;
use yewdux::prelude::*;

// --------- Domain data ----------
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
    pub technologies: Vec<String>,
    pub repoLink: String,
    pub pdfLink: String,
    pub image: String,
}

// --------- Builder layout ----------
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Layout { pub rows: Vec<Row> }

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Row { pub columns: Vec<Column> }

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Column { pub widgets: Vec<Widget> }

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Widget {
    Title { text: String, level: u8 },
    ExperienceList { filter_type: Option<String> },
    SkillsGrid { category: Option<String> },
    ProjectCard { index: usize },
    Photo { url: String, rounded: bool },
}
impl Default for Widget {
    fn default() -> Self { Self::Title { text: "Titre".into(), level: 1 } }
}

// --------- App state & store ----------
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct AppState {
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub layout: Layout,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug, Store)]
pub struct AppStore {
    pub state: AppState,
}

// --------- JSON helpers utilisés par le Builder ----------
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
                                                col.widgets.push(Widget::Title { text, level });
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
                        Widget::Title { text, level } => serde_json::json!({
                            "type": "Title", "text": text, "level": level
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
}
