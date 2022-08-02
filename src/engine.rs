use yew::prelude::*;

#[function_component(Engine)]
pub fn engine() -> Html {
    html! {<>
        <canvas id="bevy" />
        <script type="module">
            { "import init from './wasm_engine.js';
            init()" }
        </script>
        </>
    }
}

// <canvas id="bevy" width="window.innerWidth" height="window.innerHeight"/>
