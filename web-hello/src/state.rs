use std::sync::Arc;
use crate::data;
use sqlx::{Pool, Sqlite}; 

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub _experiences: Arc<Vec<data::Experience>>,
    pub projects: Arc<Vec<data::Project>>,
    pub skills: Arc<Vec<data::Skill>>,
}
