use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct TestArgs {
    map: HashMap<i32, i32>,
}

#[function_component(App)]
pub fn app() -> Html {
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let mut map = HashMap::new();
                map.insert(0, 1);

                let map_value = to_value(&TestArgs { map }).unwrap();
                web_sys::console::log_1(&map_value);

                invoke("test", map_value).await;
            });

            || {}
        },
        (),
    );

    html! {
        <main class="container">
        </main>
    }
}
