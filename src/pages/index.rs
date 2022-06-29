use crate::functions::js_glue::start_local_notification;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let text = use_state(|| String::from(""));

    let onclick = {
        let inner_text = text.clone();
        Callback::from(move |_| {
            let text = inner_text.clone();
            spawn_local(async move {
                start_local_notification().await;
                text.set(String::from("Notification set"))
            })
        })
    };

    html! {
     <div class="flex flex-col flex-grow ">
        <div class="flex flex-col flex-1 justify-center items-center">
            <button {onclick}>
                {"Press for things"}
            </button>
        </div>
        <div class="flex flex-col flex-1 items-center">
            <span>
                {text.as_str()}
            </span>
        </div>
     </div>
    }
}
