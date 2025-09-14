use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::router::{switch, Route};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Nav/>
                <main class="container py-4">
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}
