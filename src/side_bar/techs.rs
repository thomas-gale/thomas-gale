use yew::prelude::*;

#[function_component(Techs)]
pub fn techs() -> Html {
    html! {
        <div class="pb-2">
            <h2>{"Skills and Interests"}</h2>
            <p>
                <ul>
                    <li>{"Languages - Rust/Golang/C++/Typescript"}</li>
                    <li>{"Web3 Technologies - Ethereum/IPFS"}</li>
                    <li>{"Computer Graphics - WGSL Shaders"}</li>
                    <li>{"Industrial Domain - Mechanical Engineering background"}</li>
                </ul>
            </p>
        </div>
    }
}
