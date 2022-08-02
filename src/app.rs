use super::engine;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <engine::Engine/>
            <main>
                <h1>{ "Personal Website for Thomas Gale" }</h1>
                <span class="subtitle">{ "from Yew and Bevy with " }<i class="heart" /></span>
            </main>
        </>
    }
}
