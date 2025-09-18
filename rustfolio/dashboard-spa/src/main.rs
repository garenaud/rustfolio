use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::Renderer;

mod router;
mod pages;
mod components;

fn boot() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let root: Element = document
        .get_element_by_id("dashboard-root")
        .expect("#dashboard-root manquant dans le template Askama");

    Renderer::<router::App>::with_root(root).render();
}

pub fn main() { boot(); }
#[wasm_bindgen(start)]
pub fn start() { boot(); }
