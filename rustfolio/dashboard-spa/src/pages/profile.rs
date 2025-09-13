use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Me { email: String, display_name: Option<String> }

#[function_component(ProfilePage)]
pub fn profile() -> Html {
    let me = use_state(|| None::<Me>);
    {
        let me = me.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/auth/me").send().await {
                    if resp.status() == 200 {
                        if let Ok(data) = resp.json::<Me>().await {
                            me.set(Some(data));
                        }
                    }
                }
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{"Mon profil"}</h1>
            {
                if let Some(m) = &*me {
                    html! {
                        <div>
                            <p><b>{"Email : "}</b>{ &m.email }</p>
                            <p><b>{"Nom affiché : "}</b>{ m.display_name.clone().unwrap_or_default() }</p>
                        </div>
                    }
                } else {
                    html!{ <p>{"Chargement..."}</p> }
                }
            }
        </>
    }
}
