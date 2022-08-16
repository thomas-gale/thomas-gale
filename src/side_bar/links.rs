use yew::prelude::*;

#[function_component(Links)]
pub fn links() -> Html {
    html! {
        <div class="links">
            <h3>{"Clicky stuff 1"}</h3>
            <h3>{"Clicky stuff 2..n"}</h3>
        </div>
    }
}
