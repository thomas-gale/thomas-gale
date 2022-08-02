use yew::prelude::*;

#[function_component(Engine)]
pub fn engine() -> Html {
    html! {
        <div>
        <canvas id="bevy"/>
        <script type="module">
            { "import init from './wasm_engine.js';
            init()" }
        </script>
        </div>
    }
}
