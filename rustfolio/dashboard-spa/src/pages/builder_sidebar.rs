use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_save: Callback<()>,
    pub selected_row: Option<usize>,
    pub on_split: Callback<usize>,
}

#[function_component(BuilderSidebar)]
pub fn builder_sidebar(props: &Props) -> Html {
    let on_save_click = {
        let cb = props.on_save.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let split_btn = |n: usize| {
        let cb = props.on_split.clone();
        html! {
            <button
                class="builder-btn"
                onclick={Callback::from(move |_| cb.emit(n))}
                disabled={props.selected_row.is_none()}
                style="min-width:48px"
            >
                { n }
            </button>
        }
    };

    html! {
        <div class="builder-sidewrap">
            <div class="builder-panel">
                <div class="builder-panel__title">{ "Ligne" }</div>
                {
                    if let Some(id) = props.selected_row {
                        html! {
                            <>
                                <p class="builder-muted">{ format!("Sélection : Ligne #{}", id) }</p>
                                <div class="builder-grid" style="display:flex;gap:8px;flex-wrap:wrap;">
                                    <span style="opacity:.7;font-size:.9rem;">{ "Colonnes :" }</span>
                                    { split_btn(1) }
                                    { split_btn(2) }
                                    { split_btn(3) }
                                    { split_btn(4) }
                                    { split_btn(5) }
                                    { split_btn(6) }
                                </div>
                            </>
                        }
                    } else {
                        html! { <p class="builder-muted">{ "Clique une ligne pour la modifier." }</p> }
                    }
                }
            </div>

            <div class="builder-panel">
                <div class="builder-panel__title">{ "Actions" }</div>
                <button class="builder-btn" onclick={on_save_click}>{ "💾 Enregistrer le layout" }</button>
                <p class="builder-hint">{ "(Autosave plus tard)" }</p>
            </div>
        </div>
    }
}
