use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar container" style="margin-bottom:16px;">
            <Link<Route> classes="navbar__link" to={Route::Overview}>{ "Overview" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Profile}>{ "Profile" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Account}>{ "Account" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Builder}>{ "Builder" }</Link<Route>>
        </nav>
    }
}