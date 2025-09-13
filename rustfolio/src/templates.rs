use askama::Template;

// Accueil
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTpl {
    pub year: i32,
}

// Laisse ProjectsTpl et PortfolioTpl ici si tu les utilises encore :
#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTpl<'a> {
    pub year: i32,
    pub name: &'a str,
    pub title: &'a str,
    pub tagline: &'a str,
    pub projects: &'a [crate::data::Project],
}

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTpl<'a> {
    pub year: i32,
    pub name: &'a str,
    pub title: &'a str,
    pub tagline: &'a str,
}
