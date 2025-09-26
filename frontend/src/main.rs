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
        let input = input.clone();
        let chat_history_onclick: UseStateHandle<Vec<ChatMessage>> = chat_history.clone();

        
        //Callback from Send button
        Callback::from(move |_| {
            let msg = (*input).clone();

            if msg.is_empty() {
                return;
            }

            // Create a clone of chat history that includes the user message and set it immediately
            chat_history_onclick.set(
                {   let mut chat_history_with_user = (*chat_history_onclick).clone();
                    chat_history_with_user.push(ChatMessage { sender: Sender::User, text: msg.clone() });
                    chat_history_with_user
                }
            );
            // clear the input of the user in the ui in the same time too
            input.set("".to_string());



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
                
                chat_history_callback.set(
                    {   let mut chat_history_with_ai = (*chat_history_callback).clone();
                        chat_history_with_ai.push(ChatMessage { sender: Sender::AI, text: resp_json.reply.clone() });
                        chat_history_with_ai
                    }
                );
            });
        })
    };

    html! {
        <div>
            <h1>{ "AI Girlfriend Chat" }</h1>


            <div style="border: 2px solid #ccc; padding: 1rem; height: 700px;  overflow-y: scroll;">
                { for chat_history.iter().map(|m| {
                    let is_user = m.sender == Sender::User;
                    
                    html! {
                        <div style={format!(
                            "display: flex; justify-content: {}; margin: 0.5rem;",
                            
                            if is_user { "flex-end" } else { "flex-start" }
                            )
                        }>
                        
                            <div style={format!(
                                "background-color: {}; color: white; padding: 0.5rem 1rem; border-radius: 1rem; max-width: 70%;",
                                if is_user { "#007bff" } else { "#444" }
                            )}>
                                { &m.text }
                            </div>
                        
                        </div>
                    }
                })}
            </div>


            <input {oninput} type="text" value={(*input).clone()} placeholder="Type a message..." style="width: 80%;" />
            <button onclick={onclick} style="width: 19%;">{ "Send" }</button>


        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
