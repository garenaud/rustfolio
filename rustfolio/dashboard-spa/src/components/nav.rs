use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
      <nav style="display:flex; gap:1rem; padding:0.5rem 1rem; border-bottom:1px solid #ddd;">
        <Link<Route> to={Route::Profile}>{"Profil"}</Link<Route>>
        <Link<Route> to={Route::CvForm}>{"CV"}</Link<Route>>
        <Link<Route> to={Route::Builder}>{"Builder"}</Link<Route>>
        <a href="/auth/logout" onclick={
            Callback::from(|e: web_sys::MouseEvent| {
                // simple lien POST plus tard; pour l'instant on redirige
                e.prevent_default();
                web_sys::window().unwrap().location().set_href("/auth/logout").ok();
            })
        }>{"Se déconnecter"}</a>
      </nav>
    }
}
