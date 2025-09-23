use yew::prelude::*;
use yew::events::InputEvent;
use yew::TargetCast;

use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, RequestCredentials};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
struct ProfileData {
    first_name: String,
    last_name:  String,
    title:      String,
    email:      String,
    phone:      String,
    address:    String,
    city:       String,
    country:    String,
    website:    String,
    photo_url:  String,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let data    = use_state(ProfileData::default);
    let loading = use_state(|| false);
    let saving  = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);
    let ok      = use_state(|| false);

    {
        let loading = loading.clone();
        let error   = error.clone();
        let data    = data.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                let resp = Request::get("/api/cv/profile")
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
            || ()
        });
    }

    let update_text = {
        let data = data.clone();
        move |f: fn(&mut ProfileData, String)| {
            let data = data.clone();
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let mut v = (*data).clone();
                f(&mut v, input.value());
                data.set(v);
            })
        }
    };

    let update_textarea = {
        let data = data.clone();
        move |f: fn(&mut ProfileData, String)| {
            let data = data.clone();
            Callback::from(move |e: InputEvent| {
                let input: HtmlTextAreaElement = e.target_unchecked_into();
                let mut v = (*data).clone();
                f(&mut v, input.value());
                data.set(v);
            })
        }
    };

    let on_first_name = update_text(|s, v| s.first_name = v);
    let on_last_name  = update_text(|s, v| s.last_name  = v);
    let on_title      = update_text(|s, v| s.title      = v);
    let on_email      = update_text(|s, v| s.email      = v);
    let on_phone      = update_text(|s, v| s.phone      = v);
    let on_address    = update_text(|s, v| s.address    = v);
    let on_city       = update_text(|s, v| s.city       = v);
    let on_country    = update_text(|s, v| s.country    = v);
    let on_website    = update_text(|s, v| s.website    = v);
    let on_photo_url  = update_textarea(|s, v| s.photo_url  = v);

    let on_save = {
        let saving = saving.clone();
        let error  = error.clone();
        let ok     = ok.clone();
        let body   = (*data).clone();

        Callback::from(move |_| {
            let saving = saving.clone();
            let error  = error.clone();
            let ok     = ok.clone();
            let body   = body.clone();
            spawn_local(async move {
                saving.set(true);
                ok.set(false);
                error.set(None);

                let resp = Request::put("/api/cv/profile")
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&body).unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => ok.set(true),
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                saving.set(false);
            });
        })
    };

    html! {
        <section class="dash-section">
            <h2 class="dash-title">{ "Profile" }</h2>

            if let Some(err) = (*error).clone() {
                <p class="dash-error">{err}</p>
            } else if *ok {
                <p class="dash-ok">{ "Saved ✅" }</p>
            }

            <div class="dash-form">
                <input class="dash-input" type="text" placeholder="First name"
                    value={data.first_name.clone()} oninput={on_first_name} />
                <input class="dash-input" type="text" placeholder="Last name"
                    value={data.last_name.clone()} oninput={on_last_name} />
                <input class="dash-input" type="text" placeholder="Title"
                    value={data.title.clone()} oninput={on_title} />
                <input class="dash-input" type="email" placeholder="Email"
                    value={data.email.clone()} oninput={on_email} />
                <input class="dash-input" type="tel" placeholder="Phone"
                    value={data.phone.clone()} oninput={on_phone} />

                <input class="dash-input" type="text" placeholder="Address"
                    value={data.address.clone()} oninput={on_address} />
                <input class="dash-input" type="text" placeholder="City"
                    value={data.city.clone()} oninput={on_city} />
                <input class="dash-input" type="text" placeholder="Country"
                    value={data.country.clone()} oninput={on_country} />

                <input class="dash-input" type="url" placeholder="Website"
                    value={data.website.clone()} oninput={on_website} />
                <textarea class="dash-input dash-textarea" placeholder="Photo URL"
                    value={data.photo_url.clone()} oninput={on_photo_url} rows={3} />
            </div>

            <button class="dash-btn" onclick={on_save} disabled={*saving || *loading}>
                { if *saving { "Saving..." } else if *loading { "Loading..." } else { "Save" } }
            </button>
        </section>
    }
}
