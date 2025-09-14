use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav style="display:flex; gap:12px; padding:8px; border-bottom:1px solid #ddd;">
            <a href="/dashboard/">{ "Overview" }</a>
            <a href="/dashboard/profile">{ "Profile" }</a>
            <a href="/dashboard/account">{ "Account" }</a>
            <a href="/dashboard/builder">{ "Builder" }</a>
        </nav>
    }
}
