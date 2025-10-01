use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::add_row_placeholder::AddRowPlaceholder;
use crate::pages::builder_sidebar::BuilderSidebar;
use crate::store_builder::{BuilderLayout, WidgetKind};
use crate::store_cv::CVStore;

#[function_component(Builder)]
pub fn builder() -> Html {
    let (layout, dispatch) = use_store::<BuilderLayout>();
    let (cv, cv_dispatch) = use_store::<CVStore>();

    // Fetch DB au montage (garde une démo + message si ça échoue)
    {
        let cv_dispatch = cv_dispatch.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match CVStore::fetch_all().await {
                    Ok(data) => cv_dispatch.set(data),
                    Err(e) => cv_dispatch.reduce_mut(|st| {
                        st.last_error = Some(e);
                        st.source = Some("demo".into());
                        st.load_demo();
                    }),
                }
            });
            || ()
        });
    }

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
    let on_save = {
        let layout = layout.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&format!("[DEV] Layout rows={}", layout.rows.len()).into());
        })
    };

    // --------- BANNIÈRE D'ÉTAT (calculée hors du html!) ----------
    let banner: Html = {
        let src = cv.source.clone();
        let err = cv.last_error.clone();
        if src.is_some() || err.is_some() {
            html! {
                <div style="margin-bottom:10px;padding:8px 10px;border:1px solid #2a3552;border-radius:8px;background:#10192e;">
                    {
                        if let Some(s) = src {
                            html!{ <span style="opacity:.85;">{ format!("Source des données: {}", s) }</span> }
                        } else { Html::default() }
                    }
                    {
                        if let Some(e) = err {
                            html!{ <span style="color:#ff9f9f;margin-left:6px;">{ format!("— Erreur: {}", e) }</span> }
                        } else { Html::default() }
                    }
                </div>
            }
        } else {
            Html::default()
        }
    };
    // -------------------------------------------------------------

    html! {
        <div style="display:grid;grid-template-columns:280px 1fr;gap:18px;">
            <aside>
                <BuilderSidebar
                    on_save={on_save}
                    selected_row={layout.selected_row}
                    selected_column={layout.selected_column}
                    on_split={on_split}
                    on_add_widget={on_add_widget}
                />
            </aside>

            <main>
                { banner }

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
                            Callback::from(move |_e: MouseEvent| on_select_row.emit(id))
                        };

                        let n = row.columns.len().max(1);
                        let grid_style = format!(
                            "display:grid;grid-template-columns:repeat({},minmax(0,1fr));gap:12px;padding:14px;",
                            n
                        );

                        html! {
                            <section style="margin:18px 0;">
                                <div
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

                                                // IMPORTANT: stop propagation pour garder la sélection de colonne
                                                let on_col_click = {
                                                    let on_select_column = on_select_column.clone();
                                                    let id = col.id;
                                                    Callback::from(move |e: MouseEvent| {
                                                        e.stop_propagation();
                                                        on_select_column.emit(id);
                                                    })
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
                                                        <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:8px;opacity:.9;">
                                                            <span style="font-size:.85rem;">{ format!("Colonne #{}", col.id) }</span>
                                                            <span style="opacity:.6;font-size:.85rem;">{ "+" }</span>
                                                        </div>

                                                        <div style="display:flex;flex-direction:column;gap:8px;">
                                                            {
                                                                for col.widgets.iter().map(|w| {
                                                                    render_widget_preview_with_cv(w, &cv)
                                                                })
                                                            }
                                                            {
                                                                if col.widgets.is_empty() {
                                                                    html!{ <div style="opacity:.6;font-size:.9rem;">{ "Aucun contenu. Sélectionne la colonne puis ajoute un widget (sidebar)." }</div> }
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

fn render_widget_preview_with_cv(w: &WidgetKind, cv: &CVStore) -> Html {
    match w {
        WidgetKind::Text(t) => html! {
            <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:8px;">
                <div style="font-size:.8rem;opacity:.7;margin-bottom:4px;">{ "Texte" }</div>
                <div style="font-size:.95rem;color:#c8d1e6;">{ t }</div>
            </div>
        },
        WidgetKind::ProfileBasic => {
            if let Some(p) = &cv.profile {
                html! {
                    <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:10px;">
                        <div style="font-size:1rem;font-weight:700;color:#e4e9f9;">{ format!("{} {}", p.first_name, p.last_name) }</div>
                        <div style="font-size:.95rem;opacity:.85;color:#c8d1e6;">{ &p.title }</div>
                        <div style="margin-top:6px;font-size:.85rem;opacity:.7;">
                            { &p.location }{ " · " }{ &p.email }
                            { if let Some(w) = &p.website { html!{ <>{" · "}{ w }</> } } else { Html::default() } }
                        </div>
                    </div>
                }
            } else {
                html! { <div style="opacity:.7;">{ "Profil non chargé" }</div> }
            }
        },
        WidgetKind::ExperienceTimeline => {
            html! {
                <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:10px;">
                    <div style="font-size:.9rem;opacity:.85;margin-bottom:6px;">{ "Expériences" }</div>
                    <div style="display:flex;flex-direction:column;gap:8px;">
                        {
                            for cv.experiences.iter().map(|e| {
                                html!{
                                    <div style="border-left:3px solid #3a5bff;padding-left:10px;">
                                        <div style="font-weight:600;color:#e4e9f9;">{ format!("{} — {}", e.title, e.company) }</div>
                                        <div style="font-size:.85rem;opacity:.8;">{ &e.date }</div>
                                        {
                                            if !e.tasks.is_empty() {
                                                html!{ <div style="font-size:.9rem;color:#c8d1e6;margin-top:4px;">{ e.tasks.join(" • ") }</div> }
                                            } else { Html::default() }
                                        }
                                    </div>
                                }
                            })
                        }
                    </div>
                </div>
            }
        },
        WidgetKind::SkillsGrid => {
            html! {
                <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:10px;">
                    <div style="font-size:.9rem;opacity:.85;margin-bottom:6px;">{ "Compétences" }</div>
                    <div style="display:flex;flex-wrap:wrap;gap:8px;">
                        {
                            for cv.skills.iter().map(|s| {
                                html!{
                                    <span style="font-size:.8rem;opacity:.9;border:1px solid #2a3552;border-radius:6px;padding:2px 6px;">
                                        { format!("{} ({}%)", s.name, s.percentage) }
                                    </span>
                                }
                            })
                        }
                    </div>
                </div>
            }
        },
        WidgetKind::ProjectsList => {
            html! {
                <div style="background:#111a2d;border:1px solid #22304f;border-radius:8px;padding:10px;">
                    <div style="font-size:.9rem;opacity:.85;margin-bottom:6px;">{ "Projets" }</div>
                    <div style="display:flex;flex-direction:column;gap:8px;">
                        {
                            for cv.projects.iter().map(|p| {
                                html!{
                                    <div style="border:1px dashed #22304f;border-radius:8px;padding:8px;">
                                        <div style="font-weight:600;color:#e4e9f9;">{ &p.title }</div>
                                        <div style="font-size:.9rem;color:#c8d1e6;">{ &p.description }</div>
                                    </div>
                                }
                            })
                        }
                    </div>
                </div>
            }
        },
    }
}
