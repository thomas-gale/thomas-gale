use yew::prelude::*;

#[function_component(Links)]
pub fn links() -> Html {
    html! {
        <div class="pb-2">
            <a href="https://github.com/thomas-gale">
                <img src="/resources/github_icon.svg"/>
            </a>
        </div>
    }
}
