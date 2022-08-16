use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <h2>{ "Made with Yew - Github Link etc." }</h2>
        </div>
    }
}
