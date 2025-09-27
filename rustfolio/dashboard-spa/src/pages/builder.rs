use yew::prelude::*;
use yewdux::prelude::*;
use gloo_net::http::Request;
use serde_json::Value;

use crate::store::{AppStore, Row, Column, Widget};
use crate::pages::builder_sidebar::BuilderSidebar;
use crate::components::add_row_placeholder::AddRowPlaceholder;

#[function_component(BuilderPage)]
pub fn builder_page() -> Html {
    let (store, dispatch) = use_store::<AppStore>();

    {
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/layout").send().await {
                    if let Ok(json) = resp.json::<Value>().await {
                        let layout_value = json.get("layout").cloned().unwrap_or(json);
                        dispatch.reduce_mut(|s| s.state.layout_from_json(&layout_value));
                    }
                }
            });
            || ()
        });
    }

    let add_row_2cols = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                s.state.layout.rows.push(Row {
                    columns: vec![Column { widgets: vec![] }, Column { widgets: vec![] }],
                });
            });
        })
    };

    let save_layout = {
        let state = store.state.clone();
        Callback::from(move |_| {
            let layout_json = state.layout_to_json_value();
            let body = serde_json::json!({ "layout": layout_json });
            wasm_bindgen_futures::spawn_local(async move {
                let _ = Request::put("/api/layout").json(&body).unwrap().send().await;
            });
        })
    };

    html! {
        <div class="builder-shell">
            {/* SIDEBAR (1/3) */}
            <aside class="builder-sidebar">
                <BuilderSidebar on_save={save_layout.clone()} />
            </aside>

            {/* PREVIEW (2/3) */}
            <main class="builder-preview">
                <AddRowPlaceholder on_add={add_row_2cols.clone()} />

                { for store.state.layout.rows.iter().enumerate().map(|(i, row)| render_row(i, row)) }

                <AddRowPlaceholder on_add={add_row_2cols} label={Some("Ajouter une autre ligne".into())} />
            </main>
        </div>
    }
}

fn render_row(_i: usize, row: &Row) -> Html {
    html! {
      <section class="builder-row">
        <div class="builder-row__outline">
          <div class="builder-row__head">{ "Ligne" }</div>
          <div class="builder-row__cols">
            { for row.columns.iter().map(render_col) }
          </div>
        </div>
      </section>
    }
}

fn render_col(col: &Column) -> Html {
    html! {
      <div class="builder-col">
        { for col.widgets.iter().map(render_widget) }
      </div>
    }
}

fn render_widget(w: &Widget) -> Html {
    match w {
        Widget::Title { text, level, .. } => html!{ <h1>{format!("H{}: {}", level, text)}</h1> },
        Widget::ExperienceList { .. } => html!{ <div>{"[Expériences]"}</div> },
        Widget::SkillsGrid { .. } => html!{ <div>{"[Skills]"}</div> },
        Widget::ProjectCard { index } => html!{ <div>{format!("[Projet #{index}]")}</div> },
        Widget::Photo { url, .. } => html!{ <img src={url.clone()} alt="photo" /> },
    }
}
