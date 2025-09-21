use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{account, overview, builder};
use crate::pages::profile::ProfilePage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Overview,
    #[at("/profile")]
    Profile,
    #[at("/account")]
    Account,
    #[at("/builder")]
    Builder,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <overview::Overview /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::Account  => html! { <account::Account /> },
        Route::Builder  => html! { <builder::BuilderPage /> },
    }
}
