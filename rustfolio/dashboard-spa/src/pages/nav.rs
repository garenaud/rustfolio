use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar container">
            <a class="navbar__brand" href="/">{"Dashboard"}</a>
            <div class="navbar__spacer"></div>
            <Link<Route> classes="navbar__link" to={Route::Overview}>{ "Overview" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Profile}>{ "Profile" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Account}>{ "Account" }</Link<Route>>
            <Link<Route> classes="navbar__link" to={Route::Builder}>{ "Builder" }</Link<Route>>
        </nav>
    }
}