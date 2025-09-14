use yew::prelude::*;

#[function_component(Builder)]
pub fn builder() -> Html {
    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <section class="space-y-3">
                <h2 class="text-xl font-semibold">{"Sections"}</h2>
                <button class="px-3 py-1 rounded border">{"+ Expérience"}</button>
                <button class="px-3 py-1 rounded border">{"+ Compétence"}</button>
                <button class="px-3 py-1 rounded border">{"+ Projet"}</button>
            </section>
            <section class="border rounded p-4">
                <h3 class="font-semibold mb-2">{"Prévisualisation"}</h3>
                <div class="text-sm opacity-80">{"(Ici on rendra le template CV avec vos données.)"}</div>
                <div class="mt-3">
                    <button class="px-3 py-1 rounded border">{"Exporter en HTML statique"}</button>
                </div>
            </section>
        </div>
    }
}
