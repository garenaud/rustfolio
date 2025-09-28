use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Row {
    id: usize,
}

#[function_component(Builder)]
pub fn builder() -> Html {
    let rows = use_state(|| Vec::<Row>::new());

    let on_add_row = {
        let rows = rows.clone();
        Callback::from(move |_| {
            rows.set({
                let mut next = (*rows).clone();
                let id = next.len();
                next.push(Row { id });
                next
            });
        })
    };

    html! {
        <div class="builder-shell" style="display:flex;min-height:calc(100vh - 48px);">
            // Sidebar 1/3 (commentaires hors de html!)
            <aside class="builder-sidebar" style="width:32%;max-width:420px;min-width:280px;padding:16px;border-right:1px solid #25304a;background:rgba(0,0,0,0.15);">
                <h3 style="margin:0 0 12px 0;">{ "Options" }</h3>
                <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:14px;margin-bottom:12px;">
                    <div style="font-weight:600;margin-bottom:6px;">{ "Aucune sélection" }</div>
                    <p style="color:#8b93a7;margin:0;">{ "Clique une ligne (bientôt) pour la modifier ici." }</p>
                </div>
                <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;padding:14px;">
                    <div style="font-weight:600;margin-bottom:6px;">{ "Actions" }</div>
                    <button type="button" style="padding:8px 12px;border:1px solid #334155;border-radius:999px;background:transparent;cursor:pointer;">
                        { "💾 Enregistrer (à brancher)" }
                    </button>
                    <p style="color:#8b93a7;font-size:12px;margin-top:8px;">{ "(Autosave plus tard)" }</p>
                </div>
            </aside>

            // Preview 2/3
            <main class="builder-preview" style="width:68%;padding:20px 28px;">
                // Placeholder “Ajouter une ligne”
                <div
                    class="add-row"
                    onclick={on_add_row.clone()}
                    style="border:2px dashed #25304a;border-radius:14px;height:150px;display:grid;place-items:center;background:rgba(255,255,255,0.02);cursor:pointer;transition:background .12s,border-color .12s;margin:12px 0 22px;"
                >
                    <button
                        type="button"
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                        style="display:inline-flex;align-items:center;gap:10px;padding:10px 16px;border-radius:999px;border:1px solid #334155;background:#0e1523;cursor:pointer;"
                    >
                        { "＋" } <span>{ " Ajouter une ligne" }</span>
                    </button>
                </div>

                // Rendu des lignes existantes
                {
                    for (*rows).iter().map(|row| {
                        html! {
                            <section class="row" style="margin:18px 0;">
                                <div style="background:#0e1523;border:1px solid #25304a;border-radius:12px;overflow:hidden;">
                                    <div style="padding:10px 14px;font-weight:600;border-bottom:1px dashed #2a3552;">
                                        { format!("Ligne #{}", row.id) }
                                    </div>
                                    <div style="display:grid;grid-template-columns:repeat(2,1fr);gap:12px;padding:14px;">
                                        <div style="min-height:60px;border:1px dashed #2a3552;border-radius:8px;padding:10px;color:#8b93a7;">{ "Colonne A (placeholder)" }</div>
                                        <div style="min-height:60px;border:1px dashed #2a3552;border-radius:8px;padding:10px;color:#8b93a7;">{ "Colonne B (placeholder)" }</div>
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
