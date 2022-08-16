use yew::prelude::*;

#[function_component(Pic)]
pub fn pic() -> Html {
    html! {
        <img class="rounded-2xl pb-2" src={"/resources/duck_selfie.jpg"} alt={"Selfie of Thomas Gale with a duck"}/>
    }
}
