use yew::prelude::*;
use yew::Renderer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div id="dashboard-root">{ "Dashboard SPA is running." }</div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
