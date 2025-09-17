use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route; // <-- le bon chemin

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav style="display:flex; gap:12px; padding:8px; border-bottom:1px solid #ddd;">
            <Link<Route> to={Route::Overview}>{ "Overview" }</Link<Route>>
            <Link<Route> to={Route::Profile}>{ "Profile" }</Link<Route>>
            <Link<Route> to={Route::Account}>{ "Account" }</Link<Route>>
            <Link<Route> to={Route::Builder}>{ "Builder" }</Link<Route>>
        </nav>
    }
}