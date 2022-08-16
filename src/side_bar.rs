use yew::prelude::*;

mod bio;
mod links;
mod pic;
mod techs;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
        <div class="basis-full sm:basis-1/3 md:basis-1/4 lg:basis-1/5 p-4 bg-red-900">
            <h1>{ "Thomas Gale" }</h1>
            <pic::Pic/>
            <bio::Bio/>
            <links::Links/>
            <techs::Techs/>
        </div>
    }
}
