use yew::prelude::*;

#[function_component(Account)]
pub fn account() -> Html {
    html! {
        <>
            <h3>{ "Options du compte" }</h3>
            <p>{ "Ici tu gèreras email, mot de passe, suppression du compte, etc." }</p>
            <p class="text-muted">{ "On branchera cette page sur tes routes /auth prochainement." }</p>
        </>
    }
}
