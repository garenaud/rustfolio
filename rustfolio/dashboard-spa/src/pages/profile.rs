use yew::prelude::*;
use yew::events::InputEvent;
use yew::TargetCast;

use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, RequestCredentials};

use serde::{Serialize, Deserialize};
use std::rc::Rc;

/* ===================== PROFILE ===================== */

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
    location: Option<String>, // compat ancien front
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let data    = use_state(ProfileData::default);
    let loading = use_state(|| false);
    let saving  = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);
    let ok      = use_state(|| false);

    // fetch au montage
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
        <>
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

        <ExperiencesSection />
        </>
    }
}

/* ===================== EXPERIENCES ===================== */

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct TaskItem {
    id: i64,
    task: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
struct ExperienceData {
    id: Option<i64>,
    date_start: String,
    date_end: String,
    kind: String,
    title: String,
    company: String,
    location: String,
    website: String,

    #[serde(default, skip_deserializing)]
    tasks: Vec<TaskItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
struct ExperiencePayload {
    id: Option<i64>,
    date_start: String,
    date_end: String,
    kind: String,
    title: String,
    company: String,
    location: String,
    website: String,
    tasks: Vec<String>, 
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub percentage: Option<u8>,
    pub logo_url: Option<String>,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkillPayload {
    pub name: String,
    pub percentage: Option<u8>,
    pub logo_url: Option<String>,
    pub category: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct SkillOut {
    pub id: i64,
    pub name: String,
    pub percentage: Option<u8>,
    pub logo_url: Option<String>,
    pub category: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct SkillIn {
    pub name: String,
    pub percentage: Option<u8>,
    pub logo_url: Option<String>,
    pub category: Option<String>,
}

// conversion front -> payload
impl From<&ExperienceData> for ExperiencePayload {
    fn from(e: &ExperienceData) -> Self {
        Self {
            id: e.id,
            date_start: e.date_start.clone(),
            date_end: e.date_end.clone(),
            kind: e.kind.clone(),
            title: e.title.clone(),
            company: e.company.clone(),
            location: e.location.clone(),
            website: e.website.clone(),
            tasks: e.tasks.iter().map(|t| t.task.clone()).collect(),
        }
    }
}


#[function_component(ExperiencesSection)]
fn experiences_section() -> Html {
    let list = use_state(|| Vec::<ExperienceData>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let saved_id = use_state(|| Option::<i64>::None); // pour ✅

    // charge tout + tasks
    {
        let list = list.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::get("/api/cv/experiences")
                    .credentials(RequestCredentials::Include)
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        let mut exps: Vec<ExperienceData> = match r.json::<Vec<ExperienceData>>().await {
                            Ok(mut v) => {
                                for e in &mut v { e.tasks = vec![]; }
                                v
                            }
                            Err(e) => {
                                error.set(Some(format!("JSON error: {e}")));
                                vec![]
                            }
                        };

                        for e in &mut exps {
                            if let Some(id) = e.id {
                                if let Ok(resp) = Request::get(&format!("/api/cv/experiences/{id}/tasks"))
                                    .credentials(RequestCredentials::Include)
                                    .send().await
                                {
                                    if resp.ok() {
                                        if let Ok(tasks) = resp.json::<Vec<TaskItem>>().await {
                                            e.tasks = tasks;
                                        }
                                    }
                                }
                            }
                        }

                        list.set(exps);
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
            || ()
        });
    }

    // créer une expérience vide (POST) puis l’ajouter dans la liste
    let on_add = {
        let list = list.clone();
        let error = error.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            let list = list.clone();
            let error = error.clone();
            let loading = loading.clone();

            let payload = ExperiencePayload {
                date_start: "".into(),
                date_end:   "".into(),
                kind:       "".into(),
                title:      "".into(),
                company:    "".into(),
                location:   "".into(),
                website:    "".into(),
                ..Default::default()
            };

            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::post("/api/cv/experiences")
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&payload).unwrap()
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(mut created) = r.json::<ExperienceData>().await {
                            created.tasks = vec![];
                            let mut v = (*list).clone();
                            v.push(created);
                            list.set(v);
                        }
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
        })
    };

    // supprimer
    let on_delete_exp = {
        let list = list.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |exp_id: i64| {
            let list = list.clone();
            let error = error.clone();
            let loading = loading.clone();

            spawn_local(async move {
                loading.set(true);
                error.set(None);

                let resp = Request::delete(&format!("/api/cv/experiences/{exp_id}"))
                    .credentials(RequestCredentials::Include)
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        let v = (*list).clone()
                            .into_iter()
                            .filter(|e| e.id != Some(exp_id))
                            .collect::<Vec<_>>();
                        list.set(v);
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
        })
    };

    // sauver (PUT)
    let on_save_exp = {
        let error = error.clone();
        let loading = loading.clone();
        let saved_id_state = saved_id.clone();

        Callback::from(move |exp: ExperienceData| {
            let error = error.clone();
            let loading = loading.clone();
            let saved_id_state = saved_id_state.clone();
            let id = exp.id.expect("exp id manquant");
            let payload: ExperiencePayload = (&exp).into();

            spawn_local(async move {
                loading.set(true);
                error.set(None);
                saved_id_state.set(None);

                let resp = Request::put(&format!("/api/cv/experiences/{id}"))
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&payload).unwrap()
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        saved_id_state.set(Some(id)); // ✅
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }

                loading.set(false);
            });
        })
    };

    // modifs locales
    let on_change_field = {
        let list = list.clone();
        Callback::from(move |(id, field, value): (i64, &'static str, String)| {
            let mut v = (*list).clone();
            if let Some(item) = v.iter_mut().find(|e| e.id == Some(id)) {
                match field {
                    "date_start" => item.date_start = value,
                    "date_end"   => item.date_end   = value,
                    "kind"       => item.kind       = value,
                    "title"      => item.title      = value,
                    "company"    => item.company    = value,
                    "location"   => item.location   = value,
                    "website"    => item.website    = value,
                    _ => {}
                }

            }
            list.set(v);
        })
    };

    // tasks +
    let on_add_task = {
        let list = list.clone();
        let error = error.clone();
        Callback::from(move |(exp_id, text): (i64, String)| {
            let list = list.clone();
            let error = error.clone();

            spawn_local(async move {
                let resp = Request::post(&format!("/api/cv/experiences/{exp_id}/tasks"))
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&serde_json::json!({ "task": text })).unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(created) = r.json::<TaskItem>().await {
                            let mut v = (*list).clone();
                            if let Some(e) = v.iter_mut().find(|e| e.id == Some(exp_id)) {
                                e.tasks.push(created);
                            }
                            list.set(v);
                        }
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }
            });
        })
    };

    // tasks -
    let on_delete_task = {
        let list = list.clone();
        let error = error.clone();

        Callback::from(move |(exp_id, task_id): (i64, i64)| {
            let list = list.clone();
            let error = error.clone();

            spawn_local(async move {
                let resp = Request::delete(&format!("/api/cv/experiences/{exp_id}/tasks/{task_id}"))
                    .credentials(RequestCredentials::Include)
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        let mut v = (*list).clone();
                        if let Some(e) = v.iter_mut().find(|e| e.id == Some(exp_id)) {
                            e.tasks.retain(|t| t.id != task_id);
                        }
                        list.set(v);
                    }
                    Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                    Err(e) => error.set(Some(format!("Network error: {e}"))),
                }
            });
        })
    };

    html! {
        <section class="dash-section">
            <h2 class="dash-title">{ "Experiences" }</h2>

            if let Some(err) = (*error).clone() {
                <p class="dash-error">{err}</p>
            }

            <button class="dash-btn" onclick={on_add.clone()} disabled={*loading}>
                { if *loading { "..." } else { "+ Add experience" } }
            </button>

            <div class="exp-list">
                { for (*list).iter().cloned().map(|e| {
                    let key = e.id.unwrap_or_default().to_string();
                    html!{
                        <ExpItem
                            key={key}
                            exp={Rc::new(e.clone())}
                            on_change_field={on_change_field.clone()}
                            on_save={on_save_exp.clone()}
                            on_delete={on_delete_exp.clone()}
                            on_add_task={on_add_task.clone()}
                            on_delete_task={on_delete_task.clone()}
                            saved_id={(*saved_id).clone()}
                        />
                    }
                }) }
            </div>
        </section>
    }
}

/* ===================== Experience item ===================== */

#[derive(Properties, PartialEq)]
struct ExpItemProps {
    pub exp: Rc<ExperienceData>,
    pub on_change_field: Callback<(i64, &'static str, String)>,
    pub on_save: Callback<ExperienceData>,
    pub on_delete: Callback<i64>,
    pub on_add_task: Callback<(i64, String)>,
    pub on_delete_task: Callback<(i64, i64)>,
    pub saved_id: Option<i64>, // ✅
}

#[function_component(ExpItem)]
fn exp_item(props: &ExpItemProps) -> Html {
    let exp = (*props.exp).clone();
    let id = exp.id.unwrap_or_default();

    let new_task = use_state(String::default);

    let on_input_factory = {
        let cb = props.on_change_field.clone();
        move |field: &'static str| {
            let cb = cb.clone();
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                cb.emit((id, field, input.value()));
            })
        }
    };

    let on_date_start = on_input_factory("date_start");
    let on_date_end   = on_input_factory("date_end");
    let on_kind       = on_input_factory("kind");
    let on_title      = on_input_factory("title");
    let on_company    = on_input_factory("company");
    let on_location   = on_input_factory("location");
    let on_website    = on_input_factory("website");

    let do_save = {
        let on_save = props.on_save.clone();
        let exp = exp.clone();
        Callback::from(move |_| on_save.emit(exp.clone()))
    };

    let do_delete = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| on_delete.emit(id))
    };

    let on_new_task_input = {
        let st = new_task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            st.set(input.value());
        })
    };

    let do_add_task = {
        let txt = new_task.clone();
        let add = props.on_add_task.clone();
        Callback::from(move |_| {
            let t = (*txt).clone();
            if !t.trim().is_empty() {
                add.emit((id, t.clone()));
                txt.set(String::new());
            }
        })
    };

    html! {
        <div class="exp-card">
            <div class="exp-grid">
                <input class="dash-input" type="text" placeholder="Start (YYYY[-MM[-DD]])" value={exp.date_start} oninput={on_date_start} />
                <input class="dash-input" type="text" placeholder="End (YYYY[-MM[-DD]])"   value={exp.date_end}   oninput={on_date_end} />
                <input class="dash-input" type="text" placeholder="Kind"    value={exp.kind}    oninput={on_kind} />
                <input class="dash-input" type="text" placeholder="Title"   value={exp.title}   oninput={on_title} />
                <input class="dash-input" type="text" placeholder="Company" value={exp.company} oninput={on_company} />
                <input class="dash-input" type="text" placeholder="Location" value={exp.location}oninput={on_location} />
                <input class="dash-input" type="url"  placeholder="Website" value={exp.website} oninput={on_website} />
            </div>

            // tasks
            <div class="tasks">
                <div class="tasks-row">
                    <input class="dash-input" type="text" placeholder="Add a task…" value={(*new_task).clone()} oninput={on_new_task_input} />
                    <button class="dash-btn" onclick={do_add_task}>{ "+" }</button>
                </div>

                <ul class="tasks-list">
                    { for exp.tasks.iter().map(|t| {
                        let del = {
                            let cb = props.on_delete_task.clone();
                            let tid = t.id;
                            Callback::from(move |_| cb.emit((id, tid)))
                        };
                        html!{
                            <li class="task-item">
                                <span>{ &t.task }</span>
                                <button class="dash-btn dash-btn-danger" onclick={del.clone()}>{ "–" }</button>
                            </li>
                        }
                    })}
                </ul>
            </div>

            <div class="exp-actions">
                <button class="dash-btn" onclick={do_save}>{ "Save experience" }</button>
                { if props.saved_id == Some(id) {
                    html!{ <span class="dash-ok" style="margin-left: .75rem;">{ "Saved ✅" }</span> }
                } else { html!{} } }
                <button class="dash-btn dash-btn-danger" onclick={do_delete} style="margin-left: .5rem;">{ "– Delete" }</button>
            </div>
        </div>
    }
}
