use yew::prelude::*;
use crate::store::{AppStore, Row, Column, Widget};

#[function_component(Account)]
pub fn account() -> Html {
    html! {
        <>
            <h3>{ "Options du compte" }</h3>
            <p>{ "Ici tu gèreras email, mot de passe, suppression du compte, etc. a venir plus tard..." }</p>
        </>
    }
}
