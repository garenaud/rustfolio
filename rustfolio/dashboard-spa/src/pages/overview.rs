use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
        <div>
            <h2 class="text-xl font-semibold mb-2">{"Aperçu"}</h2>
            <p>{"Bienvenue sur votre dashboard."}</p>
        </div>
    }
}
