use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{Route, switch};
use crate::components::nav::Nav;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <Nav />
        <main style="padding: 1rem;">
          <Switch<Route> render={switch} />
        </main>
      </BrowserRouter>
    }
}
