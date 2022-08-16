use yew::prelude::*;

#[function_component(Bio)]
pub fn bio() -> Html {
    html! {
        <div class="bio">
            <p>{ "I'm a blah blah..." }</p>
        </div>
    }
}
