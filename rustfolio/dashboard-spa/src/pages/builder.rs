use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::add_row_placeholder::AddRowPlaceholder;
use crate::pages::builder_sidebar::BuilderSidebar;
use crate::store_builder::{BuilderLayout, WidgetKind};

#[function_component(Builder)]
pub fn builder() -> Html {
    let (layout, dispatch) = use_store::<BuilderLayout>();

    // Actions
    let on_add_row = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| dispatch.reduce_mut(|st| st.add_row()))
    };

    let on_select_row = {
        let dispatch = dispatch.clone();
        Callback::from(move |row_id: usize| dispatch.reduce_mut(|st| st.select_row(row_id)))
    };

    let on_select_column = {
        let dispatch = dispatch.clone();
        Callback::from(move |col_id: usize| dispatch.reduce_mut(|st| st.select_column(col_id)))
    };

    let on_split = {
        let dispatch = dispatch.clone();
        Callback::from(move |n: usize| dispatch.reduce_mut(|st| st.split_selected_row(n)))
    };

    let on_add_widget = {
        let dispatch = dispatch.clone();
        Callback::from(move |w: WidgetKind| dispatch.reduce_mut(|st| st.add_widget_to_selected_column(w.clone())))
    };

    // Option de sauvegarde (placeholder)
    let on_save = {
        let layout = layout.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&format!("[DEV] Sauvegarde layout JSON: rows={}", &layout.rows.len()).into());
        })
    };

    html! {
        <div class="builder-wrap" style="display:grid;grid-template-columns:280px 1fr;gap:18px;">
            <aside>
                <BuilderSidebar
                    on_save={on_save}
                    selected_row={layout.selected_row}
                    selected_column={layout.selected_column}     // NEW
                    on_split={on_split}
                    on_add_widget={on_add_widget}               // NEW
                />
            </aside>

            <main>
                // bouton/placeholder pour ajouter une ligne
                <AddRowPlaceholder on_add={Callback::from({
                    let on_add_row = on_add_row.clone();
                    move |_| on_add_row.emit(())
                })} />

                {
                    for layout.rows.iter().map(|row| {
                        let selected_row = layout.selected_row == Some(row.id);
                        let on_row_click = {
                            let on_select_row = on_select_row.clone();
                            let id = row.id;
                            Callback::from(move |_| on_select_row.emit(id))
                        };

                        let n = row.columns.len().max(1);
                        let grid_style = format!(
                            "display:grid;grid-template-columns:repeat({},minmax(0,1fr));gap:12px;padding:14px;",
                            n
                        );

                        html! {
                            <section class="row" style="margin:18px 0;">
                                <div
                                    class={classes!("row-card")}
                                    style={format!(
                                        "background:#0e1523;border:1px solid {};border-radius:12px;overflow:hidden;{}",
                                        if selected_row { "#3a5bff" } else { "#25304a" },
                                        if selected_row { "box-shadow:0 0 0 2px rgba(58,91,255,.35) inset;" } else { "" }
                                    )}
                                    onclick={on_row_click}
                                >
                                    <div style="padding:10px 14px;font-weight:600;border-bottom:1px dashed #2a3552;">
                                        { format!("Ligne #{}", row.id) } {" · "} { format!("{} colonne(s)", n) }
                                    </div>

                                    <div style={grid_style}>
                                        {
                                            for row.columns.iter().map(|col| {
                                                let selected_col = layout.selected_column == Some(col.id);
                                                let on_col_click = {
                                                    let on_select_column = on_select_column.clone();
                                                    let id = col.id;
                                                    Callback::from(move |_| on_select_column.emit(id))
                                                };

                                                html!{
                                                    <div
                                                        style={format!(
                                                            "min-height:92px;border:1px dashed {};border-radius:10px;padding:10px;color:#8b93a7;position:relative;{}",
                                                            if selected_col { "#3a5bff" } else { "#2a3552" },
                                                            if selected_col { "box-shadow:0 0 0 2px rgba(58,91,255,.25) inset;" } else { "" }
                                                        )}
                                                        onclick={on_col_click}
                                                    >
                                                        // header de colonne: bouton +
                                                        <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:8px;opacity:.9;">
                                                            <span style="font-size:.85rem;">{ format!("Colonne #{}", col.id) }</span>
                                                            <button
                                                                title="Ajouter un contenu"
                                                                style="border:1px solid #2a3552;border-radius:6px;padding:2px 8px;font-weight:600;"
                                                            >
                                                                { "+" }
                                                            </button>
                                                        </div>

                                                        // contenu des widgets (placeholder preview)
                                                        <div style="display:flex;flex-direction:column;gap:8px;">
                                                            {
                                                                for col.widgets.iter().map(|w| {
                                                                    render_widget_preview(w)
                                                                })
                                                            }
                                                            {
                                                                if col.widgets.is_empty() {
                                                                    html! { <div style="opacity:.6;font-size:.9rem;">{ "Aucun contenu. Cliquez sur la colonne puis ajoutez un widget dans la sidebar." }</div> }
                                                                } else { Html::default() }
                                                            }
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
                                    </div>
                                </div>
                            </section>
                        }
                    })
                }
            </main>
        </div>
    }
}

fn render_widget_preview(w: &WidgetKind) -> Html {
    match w {
        WidgetKind::Text(t) => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Texte" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ t }</div>
            </div>
        },
        WidgetKind::ProfileBasic => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Profil (nom + titre)" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ "Aperçu — sera peuplé depuis la DB (Profile)" }</div>
            </div>
        },
        WidgetKind::ExperienceTimeline => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Expériences" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ "Aperçu — timeline basée sur tes expériences (DB)" }</div>
            </div>
        },
        WidgetKind::SkillsGrid => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Compétences" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ "Aperçu — grilles/groupes depuis la DB Skills" }</div>
            </div>
        },
        WidgetKind::ProjectsList => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Projets" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ "Aperçu — liste de projets (DB)" }</div>
            </div>
        },
    }
}
