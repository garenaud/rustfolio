use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::Value;
//use crate::store::{AppStore, Row, Column, Widget};

#[function_component(CvFormPage)]
pub fn cv_form() -> Html {
    let data = use_state(|| Value::Object(Default::default()));

    // load on mount
    {
        let data = data.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/cv").send().await {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        // json = { ...flatten... }
                        if let Some(obj) = json.as_object() {
                            if let Some(inner) = obj.get("data") {
                                data.set(inner.clone());
                            } else {
                                data.set(json.clone());
                            }
                        } else {
                            data.set(json);
                        }
                    }
                }
            });
            || ()
        });
    }

    // save
    let on_save = {
        let data = data.clone();
        Callback::from(move |_| {
            let body = serde_json::json!({ "data": (*data).clone() });
            wasm_bindgen_futures::spawn_local(async move {
                let _ = Request::put("/api/cv").json(&body).unwrap().send().await;
            });
        })
    };

    html! {
      <div class="p-4 space-y-3">
        <h1>{"Mon CV"}</h1>
        <p class="text-muted">{"(Form champs à venir: expériences, skills, profil, etc.)"}</p>
        <button onclick={on_save}>{"💾 Enregistrer"}</button>
      </div>
    }
}
