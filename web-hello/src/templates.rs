use askama::Template;
use crate::data;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub year: i32,
    pub name: &'a str,
    pub title: &'a str,
    pub tagline: &'a str,
    pub skills: &'a [data::Skill],
    pub projects: &'a [data::Project],
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTpl<'a> {
    pub year: i32,
    pub name: &'a str,
    pub title: &'a str,
    pub tagline: &'a str,
    pub projects: &'a [data::Project],
}

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTpl<'a> {
    pub year: i32,
    pub name: &'a str,
    pub title: &'a str,
    pub tagline: &'a str,
}
