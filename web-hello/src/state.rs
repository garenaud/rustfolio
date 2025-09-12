use std::sync::Arc;
use crate::data;

#[derive(Clone)]
pub struct AppState {
    pub _experiences: Arc<Vec<data::Experience>>,
    pub projects: Arc<Vec<data::Project>>,
    pub skills: Arc<Vec<data::Skill>>,
}
