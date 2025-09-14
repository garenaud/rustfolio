use yew::prelude::*;

#[function_component(Builder)]
pub fn builder() -> Html {
    html! {
        <>
            <h3>{ "Builder de CV" }</h3>
            <p class="text-muted">
                { "Ici on ajoutera le canvas (drag & drop), les sections et l’export en page statique." }
            </p>
        </>
    }
}
