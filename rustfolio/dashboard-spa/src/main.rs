use yew::prelude::*;
use yew::Renderer;
use gloo::utils::document;

mod app;

fn main() {
    // logs + panics lisibles dans la console
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    // Monte la SPA dans la div du template Askama: <div id="dashboard-root"></div>
    let root = document().get_element_by_id("dashboard-root")
        .expect("dashboard-root missing in dashboard.html");
    Renderer::<app::App>::with_root(root).render();
}
