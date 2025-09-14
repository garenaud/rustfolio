use gloo_net::http::Request;
use web_sys::RequestCredentials;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

pub mod overview;
pub mod settings;
pub mod builder;

// petite navbar pour naviguer et vérifier que tout s'affiche
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="p-4 flex gap-4">
            <Link<Route> to={Route::Overview}>{"Overview"}</Link<Route>>
            <Link<Route> to={Route::Settings}>{"Settings"}</Link<Route>>
            <Link<Route> to={Route::Builder}>{"CV Builder"}</Link<Route>>
        </nav>
    }
}
