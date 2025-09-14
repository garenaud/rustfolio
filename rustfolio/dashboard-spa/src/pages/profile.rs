use yew::prelude::*;
use yew::events::InputEvent;
use yew::TargetCast;

use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, RequestCredentials};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
struct ProfileData {
    display_name: String,
    bio: String,
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let data    = use_state(ProfileData::default);
    let loading = use_state(|| false);
    let saving  = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);

    // GET /api/profile
    let fetch_profile = {
        let loading = loading.clone();
        let error   = error.clone();
        let data    = data.clone();

        Callback::from(move |_| {
            let loading = loading.clone();
            let error   = error.clone();
            let data    = data.clone();

            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::get("/api/profile")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        match r.json::<ProfileData>().await {
                            Ok(json) => data.set(json),
                            Err(e)   => error.set(Some(format!("JSON error: {e}"))),
                        }
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
        })
    };

    // Inputs
    let on_change_name = {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            v.display_name = input.value();
            data.set(v);
        })
    };

    let on_change_bio = {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            v.bio = input.value();
            data.set(v);
        })
    };

    // POST /api/profile
    let save_profile = {
        let saving = saving.clone();
        let error  = error.clone();
        let data   = data.clone();

        Callback::from(move |_| {
            let saving = saving.clone();
            let error  = error.clone();
            let body   = (*data).clone();

            spawn_local(async move {
                saving.set(true);
                error.set(None);

                let resp = Request::post("/api/profile")
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&body).unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {}
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                saving.set(false);
            });
        })
    };

    html! {
        <div style="padding:16px;">
            <h2>{ "Profile" }</h2>

            <button onclick={fetch_profile.clone()} disabled={*loading}>
                { if *loading { "Loading..." } else { "Load profile" } }
            </button>

            if let Some(err) = (*error).clone() {
                <p style="color:red; margin-top:8px;">{err}</p>
            }

            <div style="margin-top:12px; display:flex; flex-direction:column; gap:8px; max-width:480px;">
                <input
                    type="text"
                    placeholder="Display name"
                    value={data.display_name.clone()}
                    oninput={on_change_name}
                />
                <textarea
                    placeholder="Bio"
                    value={data.bio.clone()}
                    oninput={on_change_bio}
                    rows={6}
                />
            </div>

            <button onclick={save_profile} disabled={*saving} style="margin-top:12px;">
                { if *saving { "Saving..." } else { "Save" } }
            </button>
        </div>
    }
}
