use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Overview, Profile, Account, BuilderPage};
use crate::components::Nav;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/dashboard")]         Overview,
    #[at("/dashboard/profile")] Profile,
    #[at("/dashboard/account")] Account,
    #[at("/dashboard/builder")] Builder,
    #[not_found]
    #[at("/dashboard/404")]     NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Overview  => html!{ <Overview/> },
        Route::Profile   => html!{ <Profile/> },
        Route::Account   => html!{ <Account/> },
        Route::Builder   => html!{ <BuilderPage/> },
        Route::NotFound  => html!{ <div>{"404"}</div> },
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
