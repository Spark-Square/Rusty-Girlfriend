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

#[derive(Clone)]
struct ChatMessage {
    sender: Sender,
    text: String,
}

#[derive(Clone, PartialEq)]
enum Sender {
    User,
    AI,
}

// ------------------------------------------------

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(|| "".to_string());
    let chat_history = use_state(|| vec![]);

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let input_elem = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            input.set(input_elem.value());
        })
    };

    let onclick = {
        let input: UseStateHandle<String> = input.clone();
        let chat_history_onclick: UseStateHandle<Vec<ChatMessage>> = chat_history.clone();

        
        //Callback from Send button
        Callback::from(move |_| {
            let msg = (*input).clone();

            if msg.is_empty() {
                return;
            }

            // Create a clone of chat history that includes the user message and set it immediately
            let mut chat_history_with_curr: Vec<ChatMessage> = (*chat_history_onclick).clone();
            chat_history_with_curr.push(ChatMessage { sender: Sender::User, text: msg.clone() });
            chat_history_onclick.set(chat_history_with_curr.clone());
            input.set("".to_string());  // clear the input of the user in the ui in the same time too



            let chat_history_callback = chat_history_onclick.clone();

            // Thread to process the request to the backend
            wasm_bindgen_futures::spawn_local(async move {

                // Send to backend API
                let request_body = ChatRequest { message: msg.clone() };

                let request = Request::post("api/chat")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_body).unwrap())
                    .send();

                let response = request
                    .await
                    .unwrap();

                let resp_json: ChatResponse = response.json().await.unwrap();

                // Add AI reply to chat_history_with_user and add it to chat_history 
                chat_history_with_curr.push(ChatMessage { sender: Sender::AI, text: resp_json.reply.clone() });
                chat_history_callback.set(chat_history_with_curr);
            });
        })
    };

    html! {
        <div style="
            height: 100vh;
            display: flex;
            flex-direction: column;
            background-color: #1c023dff;
            color: white;
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        ">
            <h3 style="margin: 1rem;">{ "Rikka: Wielder of the Wicked Eye" }</h3>

            // Chat container
            <div style="
                flex: 1;
                background-color: #1c023dff;
                margin: 0 1rem 1rem 1rem;
                padding: 1rem;
                border-radius: 8px;
                overflow-y: auto;
                box-sizing: border-box;
            ">
                { for chat_history.iter().map(|m| {
                    let is_user = m.sender == Sender::User;
                    html! {
                        <div style={format!(
                            "display: flex; justify-content: {}; margin: 0.5rem 0; align-items: flex-start;",
                            if is_user { "flex-end" } else { "flex-start" }
                        )}>
                            // Avatar
                            <div style={format! ("
                                width: 40px;
                                height: 40px;
                                border-radius: 50%;
                                background-color: {};
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: white;
                                margin-right: 0.5rem;
                                flex-shrink: 0;",
                                if is_user { "#007bff" } else { "#AF69ED" }
                            )}>
                                { if is_user { "You" } else { "Ai" } }
                            </div>

                            // Chat bubble
                            <div style={format!(
                                "background-color: {}; color: white; padding: 0.5rem 1rem; border-radius: 1rem; max-width: 70%;",
                                if is_user { "#007bff" } else { "#AF69ED" }
                            )}>
                                { &m.text }
                            </div>
                        </div>
                    }
                })}
            </div>

            // Input area pinned at bottom
            <div style=
                "display: flex;
                gap: 0.25rem;
                padding: 1rem 0rem; 
                background-color: #1c023dff; 
                box-sizing: border-box;">
                <input
                    {oninput}
                    type="text"
                    value={(*input).clone()}
                    placeholder="Aa"
                    style="flex: 1; padding: 0.5rem; border-radius: 0.5rem; border: none;"
                />
                <button
                    onclick={onclick}
                    style="padding: 0.5rem 1rem; border-radius: 0.5rem; background-color: #AF69ED; color: white; border: none;"
                >
                    { "Send" }
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
