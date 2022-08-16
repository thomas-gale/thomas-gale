use yew::prelude::*;

mod bio;
mod links;
mod pic;
mod techs;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
        <div class="side_bar">
            <h1>{ "Thomas Gale" }</h1>
            <pic::Pic/>
            <bio::Bio/>
            <links::Links/>
            <techs::Techs/>
        </div>
    }
}
