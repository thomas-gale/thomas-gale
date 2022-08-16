use yew::prelude::*;

#[function_component(Name)]
pub fn name() -> Html {
    html! {
      <div class="p-2">
        <h1>{ "Thomas Gale" }</h1>
      </div>
    }
}
