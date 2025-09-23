// dashboard-spa/src/pages/experiences.rs
use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TaskDto { id: Option<i64>, task: String }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
struct Experience {
    pub id: Option<i64>,
    pub date: String,
    pub kind: String,     // <- côté API: colonne "kind"
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<String>, // utilisé pour POST/PUT main; les tâches CRUD ont leurs endpoints
}

#[derive(Clone, Debug, PartialEq, Default)]
struct ExperienceVM {
    pub id: Option<i64>,
    pub date: String,
    pub kind: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub tasks: Vec<TaskDto>, // ici on garde aussi l'ID (utile pour delete)
    pub new_task: String,
    pub saving: bool,
}

#[function_component(Experiences)]
pub fn experiences() -> Html {
    let list = use_state(Vec::<ExperienceVM>::new);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    // LOAD
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
                        let items = r.json::<Vec<Experience>>().await;
                        match items {
                            Ok(v) => {
                                // Charger aussi les tasks avec id
                                let mut out: Vec<ExperienceVM> = Vec::with_capacity(v.len());
                                for e in v {
                                    let mut vm = ExperienceVM {
                                        id: e.id,
                                        date: e.date,
                                        kind: e.kind,
                                        title: e.title,
                                        company: e.company,
                                        location: e.location,
                                        tasks: vec![],
                                        new_task: String::new(),
                                        saving: false,
                                    };
                                    if let Some(eid) = vm.id {
                                        if let Ok(tr) = Request::get(&format!("/api/cv/experiences/{eid}/tasks"))
                                            .credentials(RequestCredentials::Include)
                                            .send().await
                                        {
                                            if tr.ok() {
                                                if let Ok(ts) = tr.json::<Vec<TaskDto>>().await {
                                                    vm.tasks = ts;
                                                }
                                            }
                                        }
                                    }
                                    out.push(vm);
                                }
                                list.set(out);
                            }
                            Err(e) => error.set(Some(format!("JSON error: {e}"))),
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

    // Handlers de champ
    let set_field = {
        let list = list.clone();
        move |idx: usize, f: Box<dyn Fn(&mut ExperienceVM)>| {
            let mut v = (*list).clone();
            if let Some(item) = v.get_mut(idx) { f(item); }
            list.set(v);
        }
    };

    // ADD EXPERIENCE
    let on_add_exp = {
        let list = list.clone();
        Callback::from(move |_| {
            let mut v = (*list).clone();
            v.push(ExperienceVM::default());
            list.set(v);
        })
    };

    // SAVE EXPERIENCE
    let on_save_exp = {
        let list = list.clone();
        let error = error.clone();
        Callback::from(move |idx: usize| {
            let list = list.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut items = (*list).clone();
                if let Some(item) = items.get_mut(idx) {
                    item.saving = true; list.set(items.clone());

                    // corps à envoyer
                    let body = Experience {
                        id: item.id,
                        date: item.date.clone(),
                        kind: item.kind.clone(),
                        title: item.title.clone(),
                        company: item.company.clone(),
                        location: item.location.clone(),
                        tasks: item.tasks.iter().map(|t| t.task.clone()).collect(),
                    };

                    let result = if let Some(id) = item.id {
                        Request::put(&format!("/api/cv/experiences/{id}"))
                            .header("Content-Type", "application/json")
                            .credentials(RequestCredentials::Include)
                            .json(&body).unwrap()
                            .send().await
                    } else {
                        Request::post("/api/cv/experiences")
                            .header("Content-Type", "application/json")
                            .credentials(RequestCredentials::Include)
                            .json(&body).unwrap()
                            .send().await
                    };

                    match result {
                        Ok(r) if r.ok() => {
                            // si POST, récupérer l'id
                            if item.id.is_none() {
                                if let Ok(created) = r.json::<Experience>().await {
                                    item.id = created.id;
                                }
                            }
                        }
                        Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                        Err(e) => error.set(Some(format!("Network error: {e}"))),
                    }

                    let mut items2 = (*list).clone();
                    if let Some(it) = items2.get_mut(idx) { it.saving = false; }
                    list.set(items2);
                }
            });
        })
    };

    // ADD TASK (locale → POST si id connu)
    let on_add_task = {
        let list = list.clone();
        let error = error.clone();
        Callback::from(move |idx: usize| {
            let list = list.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut items = (*list).clone();
                if let Some(item) = items.get_mut(idx) {
                    let text = item.new_task.trim().to_string();
                    if text.is_empty() { return; }

                    // Ajout local immédiat
                    let local = TaskDto { id: None, task: text.clone() };
                    item.tasks.push(local.clone());
                    item.new_task.clear();
                    list.set(items.clone());

                    // Persister si l'exp a un id
                    if let Some(eid) = item.id {
                        let resp = Request::post(&format!("/api/cv/experiences/{eid}/tasks"))
                            .header("Content-Type", "application/json")
                            .credentials(RequestCredentials::Include)
                            .body(serde_json::json!({ "task": text }).to_string())
                            .send().await;

                        if let Ok(r) = resp {
                            if r.ok() {
                                if let Ok(saved) = r.json::<TaskDto>().await {
                                    // Remplacer la dernière task locale par la version avec id
                                    let mut items2 = (*list).clone();
                                    if let Some(it) = items2.get_mut(idx) {
                                        if let Some(last) = it.tasks.last_mut() {
                                            *last = saved;
                                        }
                                    }
                                    list.set(items2);
                                }
                            } else {
                                error.set(Some(format!("HTTP {}", r.status())));
                            }
                        }
                    }
                }
            });
        })
    };

    // DELETE TASK (nécessite l’id de la task)
    let on_del_task = {
        let list = list.clone();
        let error = error.clone();
        Callback::from(move |(idx, t_idx): (usize, usize)| {
            let list = list.clone();
            let error = error.clone();

            spawn_local(async move {
                let items = (*list).clone();
                if let Some(exp) = items.get(idx) {
                    if let (Some(eid), Some(t)) = (exp.id, exp.tasks.get(t_idx)) {
                        if let Some(tid) = t.id {
                            let resp = Request::delete(&format!("/api/cv/experiences/{eid}/tasks/{tid}"))
                                .credentials(RequestCredentials::Include)
                                .send().await;
                            match resp {
                                Ok(r) if r.ok() => {
                                    let mut items2 = (*list).clone();
                                    if let Some(e2) = items2.get_mut(idx) {
                                        e2.tasks.remove(t_idx);
                                    }
                                    list.set(items2);
                                }
                                Ok(r) => error.set(Some(format!("HTTP {}", r.status()))),
                                Err(e) => error.set(Some(format!("Network error: {e}"))),
                            }
                        }
                    }
                }
            });
        })
    };

    // RENDER
    html! {
        <div class="cv-section">
            <div class="cv-section-header">
                <h2>{"Experiences"}</h2>
                <button class="btn" onclick={on_add_exp}>{"＋ Add experience"}</button>
            </div>

            if let Some(err) = (*error).clone() {
                <p class="error">{err}</p>
            }

            if *loading {
                <p>{"Loading..."}</p>
            } else {
                { for list.iter().enumerate().map(|(i, exp)| {
                    let set = set_field.clone();

                    let on_date = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.date = v.clone()));
                        }
                    });
                    let on_kind = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.kind = v.clone()));
                        }
                    });
                    let on_title = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.title = v.clone()));
                        }
                    });
                    let on_company = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.company = v.clone()));
                        }
                    });
                    let on_location = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.location = v.clone()));
                        }
                    });
                    let on_new_task_input = Callback::from({
                        let set = set.clone();
                        move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            set(i, Box::new(move |m| m.new_task = v.clone()));
                        }
                    });

                    let save_this = {
                        let cb = on_save_exp.clone();
                        Callback::from(move |_| cb.emit(i))
                    };
                    let add_task_this = {
                        let cb = on_add_task.clone();
                        Callback::from(move |_| cb.emit(i))
                    };

                    html! {
                        <div class="card exp-card">
                            <div class="grid grid-exp">
                                <input placeholder="Date" value={exp.date.clone()} oninput={on_date} />
                                <input placeholder="Kind (work/school…)" value={exp.kind.clone()} oninput={on_kind} />
                                <input placeholder="Title" value={exp.title.clone()} oninput={on_title} />
                                <input placeholder="Company" value={exp.company.clone()} oninput={on_company} />
                                <input placeholder="Location" value={exp.location.clone()} oninput={on_location} />
                            </div>

                            <div class="tasks">
                                <div class="tasks-header">
                                    <strong>{"Tasks"}</strong>
                                </div>
                                <ul class="tasks-list">
                                    { for exp.tasks.iter().enumerate().map(|(t_i, t)| {
                                        let del = {
                                            let cb = on_del_task.clone();
                                            Callback::from(move |_| cb.emit((i, t_i)))
                                        };
                                        html! {
                                            <li class="task-item">
                                                <span>{ &t.task }</span>
                                                if t.id.is_some() {
                                                    <button class="icon-btn" onclick={del}>{"🗑"}</button>
                                                }
                                            </li>
                                        }
                                    }) }
                                </ul>
                                <div class="task-new">
                                    <input placeholder="New task…" value={exp.new_task.clone()} oninput={on_new_task_input} />
                                    <button class="btn" onclick={add_task_this}>{"Add task"}</button>
                                </div>
                            </div>

                            <div class="actions">
                                <button class="btn primary" disabled={exp.saving} onclick={save_this}>
                                    { if exp.saving { "Saving…" } else { "Save" } }
                                </button>
                            </div>
                        </div>
                    }
                }) }
            }
        </div>
    }
}
