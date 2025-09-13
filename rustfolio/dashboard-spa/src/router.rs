use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{profile::ProfilePage, cv_form::CvFormPage, builder::BuilderPage};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/dashboard")] DashboardHome,
    #[at("/dashboard/profile")] Profile,
    #[at("/dashboard/cv")] CvForm,
    #[at("/dashboard/builder")] Builder,
    #[not_found] #[at("/dashboard/404")] NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::DashboardHome => html! { <ProfilePage /> }, // page d’accueil
        Route::Profile => html! { <ProfilePage /> },
        Route::CvForm => html! { <CvFormPage /> },
        Route::Builder => html! { <BuilderPage /> },
        Route::NotFound => html! { <div>{"Page introuvable"}</div> },
    }
}
