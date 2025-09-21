use yew::prelude::*;
use yew::events::InputEvent;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, RequestCredentials};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
struct ProfileForm {
    first_name: Option<String>,
    last_name:  Option<String>,
    title:      Option<String>,
    email:      Option<String>,
    phone:      Option<String>,
    location:   Option<String>,
    bio:        Option<String>,
}

impl ProfileForm {
    fn get_s(&self, v: &Option<String>) -> String {
        v.clone().unwrap_or_default()
    }
    fn set_s(v: &mut Option<String>, s: String) {
        if s.is_empty() { *v = None; } else { *v = Some(s); }
    }
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let data    = use_state(ProfileForm::default);
    let loading = use_state(|| false);
    let saving  = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);
    let ok_msg  = use_state(|| Option::<String>::None);

    // Chargement initial
    {
        let loading = loading.clone();
        let error   = error.clone();
        let data    = data.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::get("/api/profile")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => match r.json::<ProfileForm>().await {
                        Ok(json) => data.set(json),
                        Err(e)   => error.set(Some(format!("JSON error: {e}"))),
                    },
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
            || ()
        }, ());
    }

    // Helpers pour oninput
    let set_input = |mut updater: Box<dyn FnMut(&mut ProfileForm, String)>| {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            updater.as_mut()(&mut v, input.value());
            data.set(v);
        })
    };
    let set_textarea = |mut updater: Box<dyn FnMut(&mut ProfileForm, String)>| {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            updater.as_mut()(&mut v, input.value());
            data.set(v);
        })
    };

    let on_first_name = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.first_name, val)));
    let on_last_name  = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.last_name , val)));
    let on_title      = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.title     , val)));
    let on_email      = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.email     , val)));
    let on_phone      = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.phone     , val)));
    let on_location   = set_input(Box::new(|s, val| ProfileForm::set_s(&mut s.location  , val)));
    let on_bio        = set_textarea(Box::new(|s, val| ProfileForm::set_s(&mut s.bio    , val)));

    // POST /api/profile
    let save = {
        let saving = saving.clone();
        let error  = error.clone();
        let ok_msg = ok_msg.clone();
        let body   = (*data).clone();
        Callback::from(move |_| {
            let saving = saving.clone();
            let error  = error.clone();
            let ok_msg = ok_msg.clone();
            let body   = body.clone();

            spawn_local(async move {
                saving.set(true);
                error.set(None);
                ok_msg.set(None);

                let resp = Request::post("/api/profile")
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&body).unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => ok_msg.set(Some("Profil enregistré ✅".into())),
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                saving.set(false);
            });
        })
    };

    html! {
        <div class="container">
            <h2>{ "Profile" }</h2>

            if let Some(err) = (*error).clone() {
                <p class="error">{ err }</p>
            }
            if let Some(ok) = (*ok_msg).clone() {
                <p class="ok">{ ok }</p>
            }

            <div class="form-grid">
                <input type="text"  placeholder="First name" value={data.get_s(&data.first_name)} oninput={on_first_name}/>
                <input type="text"  placeholder="Last name"  value={data.get_s(&data.last_name)}  oninput={on_last_name}/>
                <input type="text"  placeholder="Title"      value={data.get_s(&data.title)}      oninput={on_title}/>
                <input type="email" placeholder="Email"      value={data.get_s(&data.email)}      oninput={on_email}/>
                <input type="tel"   placeholder="Phone"      value={data.get_s(&data.phone)}      oninput={on_phone}/>
                <input type="text"  placeholder="Location"   value={data.get_s(&data.location)}   oninput={on_location}/>
                <textarea rows={6}   placeholder="Bio"        value={data.get_s(&data.bio)}        oninput={on_bio}/>
            </div>

            <button onclick={save} disabled={*saving}>
                { if *saving { "Saving..." } else { "Save" } }
            </button>
        </div>
    }
}
