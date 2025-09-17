use yew::prelude::*;
use yew_router::prelude::*;

// mod dashboard; // ton dossier avec mod.rs
// use dashboard::{Nav, Overview, Profile, Account, Builder};

mod router;
mod pages;
use pages::{Nav, Overview, Profile, Account, Builder};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]            Overview,   // <- routes COURTES...
    #[at("/profile")]     Profile,    // (sans /dashboard)
    #[at("/account")]     Account,
    #[at("/builder")]     Builder,
    #[not_found]
    #[at("/404")]         NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <Overview/> },
        Route::Profile  => html! { <Profile/> },
        Route::Account  => html! { <Account/> },
        Route::Builder  => html! { <Builder/> },
        Route::NotFound => html! { <div>{"Not found"}</div> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter basename="/dashboard">
            <Nav/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
