use yew::prelude::*;

use super::feed;
use super::footer;
use super::side_bar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="flex flex-col h-full">
                <div class="flex flow-row flex-wrap ju h-full">
                    <side_bar::SideBar/>
                    <feed::Feed/>
                </div>
                <footer::Footer/>
            </div>
        </main>
    }
}
