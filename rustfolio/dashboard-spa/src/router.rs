use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Overview, Profile, Account, Builder};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/dashboard")]
    Overview,
    #[at("/dashboard/profile")]
    Profile,
    #[at("/dashboard/account")]
    Account,
    #[at("/dashboard/builder")]
    Builder,
    #[not_found]
    #[at("/dashboard/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <Overview /> },
        Route::Profile  => html! { <Profile /> },
        Route::Account  => html! { <Account /> },
        Route::Builder  => html! { <Builder /> },
        Route::NotFound => html! { <div>{ "Not found" }</div> },
    }
}
