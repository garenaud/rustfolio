use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="flex items-center gap-3 border-b border-gray-700/50 p-3">
            <Link<Route> to={Route::Overview} classes="px-3 py-1 rounded hover:underline">{"Aperçu"}</Link<Route>>
            <Link<Route> to={Route::Account}  classes="px-3 py-1 rounded hover:underline">{"Compte"}</Link<Route>>
            <Link<Route> to={Route::Profile}  classes="px-3 py-1 rounded hover:underline">{"Profil/CV"}</Link<Route>>
            <Link<Route> to={Route::Builder}  classes="px-3 py-1 rounded hover:underline">{"Builder"}</Link<Route>>
        </nav>
    }
}
