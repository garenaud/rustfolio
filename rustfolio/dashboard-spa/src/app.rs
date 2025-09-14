use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <h2>{ "SPA Yew ok ✅" }</h2>
            <p>{ "Si tu lis ceci, le WASM est bien chargé." }</p>
        </div>
    }
}
