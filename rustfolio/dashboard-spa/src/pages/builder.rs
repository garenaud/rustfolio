use yew::prelude::*;

#[function_component(BuilderPage)]
pub fn builder() -> Html {
    html! {
        <>
          <h1>{"Builder"}</h1>
          <p>{"Prototype: grille d’éléments (sections / colonnes / modules), drag & drop plus tard."}</p>
          <div style="display:grid; grid-template-columns: 1fr 3fr; gap:1rem;">
            <aside style="border:1px solid #ddd; padding:0.5rem;">
              <h3>{"Modules"}</h3>
              <ul>
                <li>{"En-tête"}</li>
                <li>{"Texte"}</li>
                <li>{"Carte d’expérience"}</li>
                <li>{"Liste de compétences"}</li>
              </ul>
            </aside>
            <section style="border:1px solid #ddd; padding:0.5rem; min-height:300px;">
              <p>{"Zone de composition (droppable) — on y rendra un arbre d’éléments."}</p>
            </section>
          </div>
        </>
    }
}
