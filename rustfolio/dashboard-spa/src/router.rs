use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Overview, Profile, Account, Builder};
use crate::components::Nav;   // <— PAS depuis pages


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/dashboard")]         Overview,
    #[at("/dashboard/profile")] Profile,
    #[at("/dashboard/account")] Account,
    #[at("/dashboard/builder")] Builder,
    #[not_found]
    #[at("/dashboard/404")]     NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <Overview /> },
        Route::Profile  => html! { <Profile  /> },
        Route::Account  => html! { <Account  /> },
        Route::Builder  => html! { <Builder  /> },
        Route::NotFound => html! { <div class="p-4">{"Page introuvable"}</div> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="dashboard-shell">
                <Nav />
                <main class="dashboard-main">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}
