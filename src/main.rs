mod app;
mod feed;
mod footer;
mod side_bar;

use app::App;

fn main() {
    yew::start_app::<App>();
}
