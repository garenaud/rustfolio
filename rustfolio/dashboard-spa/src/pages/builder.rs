use yew::prelude::*;
use yewdux::prelude::*;
use gloo_net::http::Request;
use serde_json::Value;

use crate::store::{AppStore, Row, Column, Widget};

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
                        dispatch.reduce_mut(|s| {
                            s.state.layout_from_json(&layout_value);
                        });
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
                    columns: vec![
                        Column { widgets: vec![] },
                        Column { widgets: vec![] },
                    ]
                });
            });
        })
    };

    let add_title = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                if let Some(last_row) = s.state.layout.rows.last_mut() {
                    if let Some(first_col) = last_row.columns.first_mut() {
                        first_col.widgets.push(Widget::Title { text: "Mon CV".into(), level: 1 });
                    }
                }
            });
        })
    };

    let save_layout = {
        let layout_json = store.state.to_layout_json();
        Callback::from(move |_| {
            let body = serde_json::json!({ "layout": layout_json });
            wasm_bindgen_futures::spawn_local(async move {
                let _ = Request::put("/api/layout").json(&body).unwrap().send().await;
            });
        })
    };

    html! {
      <div class="grid grid-cols-12 gap-4 p-6">
        <div class="col-span-8 border rounded p-4">
          { for store.state.layout.rows.iter().map(render_row) }

          <div class="mt-4 space-x-2">
            <button onclick={add_row_2cols}>{"➕ Rangée (2 colonnes)"}</button>
            <button onclick={add_title}>{"➕ Titre (col1 dernière rangée)"}</button>
          </div>
        </div>

        <div class="col-span-4 border rounded p-4 space-y-2">
          <h3>{"Actions"}</h3>
          <button onclick={save_layout}>{"💾 Enregistrer le layout"}</button>
          <p class="text-sm text-gray-500">{"(On branchera l’autosave plus tard)"}</p>
        </div>
      </div>
    }
}

fn render_row(row: &Row) -> Html {
    html! {
      <div class="grid grid-cols-2 gap-4 mb-4">
        { for row.columns.iter().map(render_col) }
      </div>
    }
}

fn render_col(col: &Column) -> Html {
    html! {
      <div class="min-h-[120px] border rounded p-3">
        { for col.widgets.iter().map(render_widget) }
      </div>
    }
}

fn render_widget(w: &Widget) -> Html {
    match w {
        Widget::Title { text, level } => html!{ <h1>{format!("H{}: {}", level, text)}</h1> },
        Widget::ExperienceList { .. } => html!{ <div>{"[Expériences]"}</div> },
        Widget::SkillsGrid { .. } => html!{ <div>{"[Skills]"}</div> },
        Widget::ProjectCard { index } => html!{ <div>{format!("[Projet #{index}]")}</div> },
        Widget::Photo { url, .. } => html!{ <img src={url.clone()} /> },
    }
}
