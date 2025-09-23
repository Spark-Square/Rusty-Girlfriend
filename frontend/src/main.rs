use yew::prelude::*;
use gloo::net::http::Request;
use serde::{Deserialize, Serialize};


// ----------------- DATA STRUCTS -----------------

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    reply: String,
}

// ------------------------------------------------

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(|| "".to_string());
    let messages = use_state(|| vec![]);

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let input_elem = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            input.set(input_elem.value());
        })
    };

    let onclick = {
        let input = input.clone();
        let messages = messages.clone();
        Callback::from(move |_| {
            let msg = (*input).clone();
            if msg.is_empty() {
                return;
            }

            let messages = messages.clone();
            let input = input.clone();

            wasm_bindgen_futures::spawn_local(async move {

                // Send to backend API
                let request_body = ChatRequest { message: msg.clone() };

                let resp = Request::post("http://localhost:8000/api/chat")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_body).unwrap())
                    .send()
                    .await
                    .unwrap();

                let resp_json: ChatResponse = resp.json().await.unwrap();

                // Update chat messages
                messages.set({
                    let mut new_msgs = (*messages).clone();
                    new_msgs.push(format!("You: {}", msg));
                    new_msgs.push(format!("AI: {}", resp_json.reply));
                    new_msgs
                });

                input.set("".to_string());
            });
        })
    };

    html! {
        <div>
            <h1>{ "AI Girlfriend Chat" }</h1>


            <div style="border: 2px solid #ccc; padding: 1rem; height: 700px; overflow-y: scroll;">
                { for messages.iter().map(|m| html! { <p>{ m }</p> }) }
            </div>


            <input {oninput} type="text" value={(*input).clone()} placeholder="Type a message..." style="width: 80%;" />
            <button onclick={onclick} style="width: 19%;">{ "Send" }</button>


        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
