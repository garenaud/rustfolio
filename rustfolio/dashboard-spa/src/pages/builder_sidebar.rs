use yew::prelude::*;
use crate::store_builder::WidgetKind;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_save: Callback<()>,
    pub selected_row: Option<usize>,
    pub selected_column: Option<usize>,
    pub on_split: Callback<usize>,
    pub on_add_widget: Callback<WidgetKind>,
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
                onclick={Callback::from(move |_| cb.emit(n))}
                disabled={props.selected_row.is_none()}
                style="min-width:48px;border:1px solid #25304a;border-radius:8px;padding:6px 10px;"
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
        <div style="display:flex;flex-direction:column;gap:14px;">
            <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div style="font-weight:700;margin-bottom:8px;">{ "Ligne" }</div>
                {
                    if let Some(id) = props.selected_row {
                        html! {
                            <>
                                <p style="opacity:.7;margin-bottom:8px;">{ format!("Sélection : Ligne #{}", id) }</p>
                                <div style="display:flex;gap:8px;flex-wrap:wrap;">
                                    <span style="opacity:.7;font-size:.9rem;line-height:28px;">{ "Colonnes :" }</span>
                                    { split_btn(1) }{ split_btn(2) }{ split_btn(3) }
                                    { split_btn(4) }{ split_btn(5) }{ split_btn(6) }
                                </div>
                            </>
                        }
                    } else {
                        html! { <p style="opacity:.7;">{ "Clique une ligne pour la modifier." }</p> }
                    }
                }
            </div>

            <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div style="font-weight:700;margin-bottom:8px;">{ "Contenu de colonne" }</div>
                {
                    if let Some(col_id) = props.selected_column {
                        html! {
                            <>
                                <p style="opacity:.7;margin-bottom:8px;">{ format!("Sélection : Colonne #{}", col_id) }</p>
                                <div style="display:flex;flex-direction:column;gap:8px;">
                                    <button onclick={add(WidgetKind::Text("Texte libre…".into()))}>{ "➕ Texte libre" }</button>
                                    <button onclick={add(WidgetKind::ProfileBasic)}>{ "➕ Profil (nom + titre)" }</button>
                                    <button onclick={add(WidgetKind::ExperienceTimeline)}>{ "➕ Expériences (timeline)" }</button>
                                    <button onclick={add(WidgetKind::SkillsGrid)}>{ "➕ Compétences (grille)" }</button>
                                    <button onclick={add(WidgetKind::ProjectsList)}>{ "➕ Projets (liste)" }</button>
                                </div>
                                <p style="opacity:.6;font-size:.85rem;margin-top:8px;">
                                    { "Ces widgets utiliseront tes données DB (profil, expériences, compétences, projets)." }
                                </p>
                            </>
                        }
                    } else {
                        html! { <p style="opacity:.7;">{ "Clique une colonne (bordure bleue), puis ajoute un widget." }</p> }
                    }
                }
            </div>

            <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:12px;">
                <div style="font-weight:700;margin-bottom:8px;">{ "Actions" }</div>
                <button onclick={on_save_click}>{ "💾 Enregistrer le layout" }</button>
                <p style="opacity:.6;font-size:.85rem;">{ "(Autosave à venir)" }</p>
            </div>
        </div>
    }
}
