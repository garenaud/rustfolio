use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_save: Callback<()>,
}

#[function_component(BuilderSidebar)]
pub fn builder_sidebar(props: &Props) -> Html {
    let on_save_click = {
        let cb = props.on_save.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class="builder-sidewrap">
            <h3 class="builder-side__title">{ "Options" }</h3>

            <div class="builder-panel">
                <div class="builder-panel__title">{ "Aucune sélection" }</div>
                <p class="builder-muted">{ "Clique une ligne/colonne (bientôt) pour la modifier ici." }</p>
            </div>

            <div class="builder-panel">
                <div class="builder-panel__title">{ "Actions" }</div>
                <button class="builder-btn" onclick={on_save_click}>{ "💾 Enregistrer le layout" }</button>
                <p class="builder-hint">{ "(Autosave plus tard)" }</p>
            </div>
        </div>
    }
}
