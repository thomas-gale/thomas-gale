use yew::prelude::*;

mod post;

#[function_component(Feed)]
pub fn feed() -> Html {
    let posts = vec![
        post::PostProps {
            title: String::from("Hello World"),
            date: String::from("10/08/2022"),
            content: String::from("Hardcoded, test blog content"),
        },
        post::PostProps {
            title: String::from("Hello World 2"),
            date: String::from("12/08/2022"),
            content: String::from("Second hardcoded, test blog content"),
        },
    ];

    html! {
        <div class="flex-auto p-4 bg-blue-900">
            <h2>{ "Main Feed - Random Musings" }</h2>
            {
                posts.into_iter().map(|post_props| {
                    html! { <post::Post title={post_props.title.clone()} date={post_props.date.clone()} content={post_props.content.clone()} /> }
                }).collect::<Html>()
            }
        </div>
    }
}
