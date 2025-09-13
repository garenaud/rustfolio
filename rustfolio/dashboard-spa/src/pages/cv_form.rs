use yew::prelude::*;

#[function_component(CvFormPage)]
pub fn cv_form() -> Html {
    html! {
        <>
          <h1>{"Mon CV"}</h1>
          <p>{"Ici on mettra le formulaire (expériences, compétences, éducation...)"}</p>
          // TODO: champs contrôlés + POST/PUT vers /api/cv (à créer côté back)
        </>
    }
}
