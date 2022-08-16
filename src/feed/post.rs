use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub title: String,
    pub date: String,
    pub content: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    html! {
        <div key={props.title.clone()} class="post">
            <h2>{ props.title.clone() }</h2>
            <h2>{ props.date.clone() }</h2>
            <p>{ props.content.clone() }</p>
        </div>
    }
}
