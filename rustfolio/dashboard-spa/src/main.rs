use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter basename="/dashboard">
            <pages::Nav />
            <Switch<router::Route> render={router::switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
