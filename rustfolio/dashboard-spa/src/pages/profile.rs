use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div class="space-y-3 max-w-3xl">
            <h2 class="text-xl font-semibold">{"Données CV"}</h2>
            <label class="block">
                <span>{"Titre (ex: Développeur Rust)"}</span>
                <input class="block w-full p-2 rounded bg-transparent border" />
            </label>
            <label class="block">
                <span>{"Résumé"}</span>
                <textarea class="block w-full p-2 rounded bg-transparent border" rows=5 />
            </label>
            <div class="flex gap-2">
                <button type="button" class="px-4 py-2 rounded border hover:bg-white/10">{"Sauver"}</button>
                <button type="button" class="px-4 py-2 rounded border hover:bg-white/10">{"Charger données existantes"}</button>
            </div>
        </div>
    }
}
