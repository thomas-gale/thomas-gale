use yew::prelude::*;

mod bio;
mod links;
mod name;
mod pic;
mod techs;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
        <div class="basis-full sm:basis-1/3 md:basis-1/4 lg:basis-1/5 h-min sm:h-full p-2 bg-red-900">
            <name::Name/>
            <pic::Pic/>
            <bio::Bio/>
            <links::Links/>
            <techs::Techs/>
        </div>
    }
}
