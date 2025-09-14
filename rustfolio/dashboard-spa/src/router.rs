use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Overview, Account, Profile, Builder};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/dashboard")]
    Overview,
    #[at("/dashboard/account")]
    Account,
    #[at("/dashboard/profile")]
    Profile,
    #[at("/dashboard/builder")]
    Builder,
    // fallback pour sous-routes inconnues
    #[not_found]
    #[at("/dashboard/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <Overview /> },
        Route::Account  => html! { <Account /> },
        Route::Profile  => html! { <Profile /> },
        Route::Builder  => html! { <Builder /> },
        Route::NotFound => html! { <p>{"Not found"}</p> },
    }
}
