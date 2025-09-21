use yew::prelude::*;
use yew::events::InputEvent;
use yew::TargetCast;

use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
struct Profile {
    first_name: String,
    last_name: String,
    title: String,
    email: String,
    phone: String,
    location: String,
    about: String,
    website: Option<String>,
    github: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>,
    picture: Option<String>,
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let data    = use_state(Profile::default);
    let loading = use_state(|| false);
    let saving  = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);
    let ok_msg  = use_state(|| Option::<String>::None);

    // charge au mount: GET /api/cv/profile
    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::get("/api/cv/profile").send().await;
                match resp {
                    Ok(r) if r.ok() => match r.json::<Profile>().await {
                        Ok(p) => data.set(p),
                        Err(e) => error.set(Some(format!("JSON error: {e}"))),
                    },
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
            || ()
        });
    }

    // helpers d’update contrôlé
    let update_text = |f: fn(&mut Profile, String)| {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            f(&mut v, input.value());
            data.set(v);
        })
    };
    let update_textarea = |f: fn(&mut Profile, String)| {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            f(&mut v, input.value());
            data.set(v);
        })
    };
    let update_opt = |f: fn(&mut Profile, Option<String>)| {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut v = (*data).clone();
            let val = input.value();
            f(&mut v, if val.trim().is_empty() { None } else { Some(val) });
            data.set(v);
        })
    };

    // PUT /api/cv/profile (remplacement complet)
    let save = {
        let data = data.clone();
        let saving = saving.clone();
        let error = error.clone();
        let ok_msg = ok_msg.clone();
        Callback::from(move |_| {
            let body = (*data).clone();
            let saving = saving.clone();
            let error = error.clone();
            let ok_msg = ok_msg.clone();
            spawn_local(async move {
                saving.set(true);
                error.set(None);
                ok_msg.set(None);

                let resp = Request::put("/api/cv/profile")
                    .header("Content-Type", "application/json")
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
        <div class="p-4 space-y-4">
            <h2 class="text-2xl font-bold">{"Profile"}</h2>

            if let Some(err) = (*error).clone() {
                <p class="text-red-600">{err}</p>
            }
            if let Some(ok) = (*ok_msg).clone() {
                <p class="text-green-600">{ok}</p>
            }

            <fieldset class="grid grid-cols-1 md:grid-cols-2 gap-3 max-w-3xl" disabled={*loading || *saving}>
                <input class="border p-2 rounded" type="text" placeholder="First name"
                    value={(*data).first_name.clone()}
                    oninput={update_text(|p,v| p.first_name = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Last name"
                    value={(*data).last_name.clone()}
                    oninput={update_text(|p,v| p.last_name = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Title"
                    value={(*data).title.clone()}
                    oninput={update_text(|p,v| p.title = v)} />
                <input class="border p-2 rounded" type="email" placeholder="Email"
                    value={(*data).email.clone()}
                    oninput={update_text(|p,v| p.email = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Phone"
                    value={(*data).phone.clone()}
                    oninput={update_text(|p,v| p.phone = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Location"
                    value={(*data).location.clone()}
                    oninput={update_text(|p,v| p.location = v)} />

                <textarea class="border p-2 rounded col-span-full" rows={6} placeholder="About"
                    value={(*data).about.clone()}
                    oninput={update_textarea(|p,v| p.about = v)} />

                <input class="border p-2 rounded" type="text" placeholder="Website"
                    value={(*data).website.clone().unwrap_or_default()}
                    oninput={update_opt(|p,v| p.website = v)} />
                <input class="border p-2 rounded" type="text" placeholder="GitHub"
                    value={(*data).github.clone().unwrap_or_default()}
                    oninput={update_opt(|p,v| p.github = v)} />
                <input class="border p-2 rounded" type="text" placeholder="LinkedIn"
                    value={(*data).linkedin.clone().unwrap_or_default()}
                    oninput={update_opt(|p,v| p.linkedin = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Twitter"
                    value={(*data).twitter.clone().unwrap_or_default()}
                    oninput={update_opt(|p,v| p.twitter = v)} />
                <input class="border p-2 rounded" type="text" placeholder="Picture URL"
                    value={(*data).picture.clone().unwrap_or_default()}
                    oninput={update_opt(|p,v| p.picture = v)} />
            </fieldset>

            <div class="pt-2">
                <button class="border px-3 py-2 rounded"
                    onclick={save}
                    disabled={*saving}>
                    { if *saving { "Saving..." } else { "Save" } }
                </button>
                { " " }
                { if *loading { html!{ <span>{"Loading profile..."}</span> } } else { html!{} } }
            </div>
        </div>
    }
}
