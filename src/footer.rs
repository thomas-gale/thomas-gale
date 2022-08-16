use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="flex-auto p-2 bg-green-900">
            <h2>{ "Made with Yew - Github Link etc." }</h2>
        </div>
    }
}
