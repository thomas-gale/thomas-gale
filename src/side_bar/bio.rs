use yew::prelude::*;

#[function_component(Bio)]
pub fn bio() -> Html {
    html! {
        <div class="pb-4">
            <h2>{"Bio"}</h2>
            <p>{ "Software Engineer (generalist) interesting in building tools, concepts and experiences to help improve humanity." }</p>
        </div>
    }
}
