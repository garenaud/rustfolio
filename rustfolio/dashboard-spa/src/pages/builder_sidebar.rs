use yew::prelude::*;
use crate::store_builder::WidgetKind;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_save: Callback<()>,
    pub selected_row: Option<usize>,
    pub selected_column: Option<usize>,     // NEW
    pub on_split: Callback<usize>,
    pub on_add_widget: Callback<WidgetKind>, // NEW
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

    let add = |w: WidgetKind| {
        let cb = props.on_add_widget.clone();
        Callback::from(move |_| cb.emit(w.clone()))
    };

    html! {
        <div class="builder-sidewrap" style="display:flex;flex-direction:column;gap:14px;">
            <div class="builder-panel" style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div class="builder-panel__title" style="font-weight:700;margin-bottom:8px;">{ "Ligne" }</div>
                {
                    if let Some(id) = props.selected_row {
                        html! {
                            <>
                                <p class="builder-muted" style="opacity:.7;margin-bottom:8px;">{ format!("Sélection : Ligne #{}", id) }</p>
                                <div class="builder-grid" style="display:flex;gap:8px;flex-wrap:wrap;">
                                    <span style="opacity:.7;font-size:.9rem;line-height:28px;">{ "Colonnes :" }</span>
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
                        html! { <p class="builder-muted" style="opacity:.7;">{ "Clique une ligne pour la modifier." }</p> }
                    }
                }
            </div>

            <div class="builder-panel" style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div class="builder-panel__title" style="font-weight:700;margin-bottom:8px;">{ "Contenu de colonne" }</div>
                {
                    if let Some(col_id) = props.selected_column {
                        html! {
                            <>
                                <p class="builder-muted" style="opacity:.7;margin-bottom:8px;">{ format!("Sélection : Colonne #{}", col_id) }</p>
                                <div style="display:flex;flex-direction:column;gap:8px;">
                                    <button class="builder-btn" onclick={add(WidgetKind::Text("Texte libre…".into()))}>{ "➕ Texte libre" }</button>
                                    <button class="builder-btn" onclick={add(WidgetKind::ProfileBasic)}>{ "➕ Profil (nom + titre)" }</button>
                                    <button class="builder-btn" onclick={add(WidgetKind::ExperienceTimeline)}>{ "➕ Expériences (timeline)" }</button>
                                    <button class="builder-btn" onclick={add(WidgetKind::SkillsGrid)}>{ "➕ Compétences (grille)" }</button>
                                    <button class="builder-btn" onclick={add(WidgetKind::ProjectsList)}>{ "➕ Projets (liste)" }</button>
                                </div>
                                <p class="builder-hint" style="opacity:.6;font-size:.85rem;margin-top:8px;">
                                    { "Ces widgets seront peuplés par tes données (onglet Profil/Expériences/Compétences/Projets)." }
                                </p>
                            </>
                        }
                    } else {
                        html! { <p class="builder-muted" style="opacity:.7;">{ "Clique une colonne (bordure bleue) puis ajoute un widget." }</p> }
                    }
                }
            </div>

            <div class="builder-panel" style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div class="builder-panel__title" style="font-weight:700;margin-bottom:8px;">{ "Actions" }</div>
                <button class="builder-btn" onclick={on_save_click}>{ "💾 Enregistrer le layout" }</button>
                <p class="builder-hint" style="opacity:.6;font-size:.85rem;">{ "(Autosave à venir)" }</p>
            </div>
        </div>
    }
}
