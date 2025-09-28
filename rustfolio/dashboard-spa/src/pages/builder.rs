use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::add_row_placeholder::AddRowPlaceholder;
use crate::pages::builder_sidebar::BuilderSidebar;
use crate::store_builder::BuilderLayout;

#[function_component(Builder)]
pub fn builder() -> Html {
    // Yewdux Store
    let (layout, dispatch) = use_store::<BuilderLayout>();

    // Actions
    let on_add_row = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|st| st.add_row());
        })
    };

    let on_select_row = {
        let dispatch = dispatch.clone();
        Callback::from(move |row_id: usize| {
            dispatch.reduce_mut(|st| st.select_row(row_id));
        })
    };

    let on_split = {
        let dispatch = dispatch.clone();
        Callback::from(move |n: usize| {
            dispatch.reduce_mut(|st| st.split_selected_row(n));
        })
    };

    // Option de sauvegarde (placeholder pour l’instant)
    let on_save = {
        let layout = layout.clone();
        Callback::from(move |_| {
            // Plus tard: POST vers ton API.
            web_sys::console::log_1(&format!("[DEV] Sauvegarde layout JSON: {:?}", &layout.rows.len()).into());
        })
    };

    html! {
        <div class="builder-wrap" style="display:grid;grid-template-columns:280px 1fr;gap:18px;">
            <aside>
                <BuilderSidebar
                    on_save={on_save}
                    selected_row={layout.selected_row}
                    on_split={on_split}
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
                        let selected = layout.selected_row == Some(row.id);
                        let on_click = {
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
                                    class={classes!(
                                        "row-card"
                                    )}
                                    style={format!(
                                        "background:#0e1523;border:1px solid {};border-radius:12px;overflow:hidden;{}",
                                        if selected { "#3a5bff" } else { "#25304a" },
                                        if selected { "box-shadow:0 0 0 2px rgba(58,91,255,.35) inset;" } else { "" }
                                    )}
                                    onclick={on_click}
                                >
                                    <div style="padding:10px 14px;font-weight:600;border-bottom:1px dashed #2a3552;">
                                        { format!("Ligne #{}", row.id) } {" · "} { format!("{} colonne(s)", n) }
                                    </div>

                                    <div style={grid_style}>
                                        {
                                            for row.columns.iter().map(|_col| {
                                                html!{
                                                    <div style="min-height:68px;border:1px dashed #2a3552;border-radius:10px;padding:10px;color:#8b93a7;">
                                                        { "Colonne (placeholder)" }
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
