use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddRowPlaceholderProps {
    pub on_add: Callback<()>,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component(AddRowPlaceholder)]
pub fn add_row_placeholder(props: &AddRowPlaceholderProps) -> Html {
    let onclick_box = {
        let on_add = props.on_add.clone();
        Callback::from(move |_| on_add.emit(()))
    };

    html! {
        <div class="builder-addrow" onclick={onclick_box}>
            <button
                class="builder-addrow__btn"
                type="button"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                { "＋" }
                <span>{ props.label.clone().unwrap_or_else(|| "Ajouter une ligne".into()) }</span>
            </button>
        </div>
    }
}
