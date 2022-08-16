use yew::prelude::*;

use super::feed;
use super::footer;
use super::side_bar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="v-container">
                <div class="h-container">
                    <side_bar::SideBar/>
                    <feed::Feed/>
                </div>
                <footer::Footer/>
            </div>
        </main>
    }
}
