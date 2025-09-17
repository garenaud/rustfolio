use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
            <section class="card p-8">
                <h2 class="text-2xl font-bold mb-4">{"Overview"}</h2>
                <p>{"Bienvenue sur ton dashboard."}</p>
            </section>
    }
}