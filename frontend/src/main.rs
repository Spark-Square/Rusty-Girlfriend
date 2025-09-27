use yew::prelude::*;
use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::{Function, Object, Reflect};


// ----------------- DATA STRUCTS -----------------

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    reply: String,
}

#[derive(Clone, PartialEq)]
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

    // NodeRef for chat container
    let chat_container_ref = use_node_ref().clone();

    // Auto-scroll when chat_history updates   
    let chat_container_ref_clone = chat_container_ref.clone(); // make a clone for the closure
        use_effect_with(
        chat_history.clone(),
        move |_| {
            if let Some(div) = chat_container_ref_clone.cast::<web_sys::Element>() {
                // build { top: <height>, behavior: "smooth" }
                let opts = Object::new();
                Reflect::set(&opts, &JsValue::from_str("top"), &JsValue::from_f64(div.scroll_height() as f64)).ok();
                Reflect::set(&opts, &JsValue::from_str("behavior"), &JsValue::from_str("smooth")).ok();

                // call element.scrollTo(opts) if available
                let elem_js = JsValue::from(div.clone());
                if let Ok(scroll_to) = Reflect::get(&elem_js, &JsValue::from_str("scrollTo")) {
                    if let Some(func) = scroll_to.dyn_ref::<Function>() {
                        let _ = func.call1(&elem_js, &opts.into());
                    } else {
                        // fallback: instant scroll
                        div.set_scroll_top(div.scroll_height());
                    }
                } else {
                    // fallback: instant scroll
                    div.set_scroll_top(div.scroll_height());
                }
            }
            || ()
        },
    );

    

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let input_elem = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            input.set(input_elem.value());
        })
    };

    fn send_message (input: &UseStateHandle<String>, chat_history_onevent: &UseStateHandle<Vec<ChatMessage>>) {
            let msg = (*input).clone();

            if msg.is_empty() {
                return;
            }

            // Create a clone of chat history that includes the user message and set it immediately
            let mut chat_history_with_curr: Vec<ChatMessage> = (*chat_history_onevent).to_vec();
            chat_history_with_curr.push(ChatMessage { sender: Sender::User, text: msg.to_string() });
            chat_history_onevent.set(chat_history_with_curr.clone());
            input.set("".to_string());  // clear the input of the user in the ui in the same time too



            let chat_history_callback = chat_history_onevent.clone();

            // Thread to process the request to the backend
            wasm_bindgen_futures::spawn_local(async move {

                // Send to backend API
                let request_body = ChatRequest { message: msg.to_string() };

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
    }
    

    let onclick = {
        let input: UseStateHandle<String> = input.clone();
        let chat_history_onclick: UseStateHandle<Vec<ChatMessage>> = chat_history.clone();
        Callback::from(move |_: MouseEvent| {
            send_message(&input, &chat_history_onclick);
        })
        
    };

        // Enter to send message
    let onkeypress = {
        let input = input.clone();
        let chat_history_onkeypress = chat_history.clone();
        Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
            send_message(&input, &chat_history_onkeypress);
        }
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
            <div 
                ref = {chat_container_ref} // attach NodeRef
                style="
                flex: 1;
                background-color: #1c023dff
;
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
                            // Render bubble first if user, avatar second
                            { if is_user {
                                html! {
                                    <>
                                        // Chat bubble
                                        <div style={format!(
                                        "background-color: {}; color: white; padding: 0.5rem 1rem; border-radius: 1rem; max-width: 70%;",
                                        "#007bff"
                                        )}>
                                            { &m.text }
                                        </div>

                                        // User avatar on the right
                                        <img src="/Icons/Yuuta.jpg" 
                                            alt="Me" 
                                            style="
                                            width: 40px;
                                            height: 40px;
                                            border-radius: 50%;
                                            margin-left: 0.5rem;
                                            flex-shrink: 0;
                                        "/>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <img src="/Icons/Rikka.jpg" 
                                            alt="Ai" 
                                            style="
                                            width: 40px;
                                            height: 40px;
                                            border-radius: 50%;
                                            margin-right: 0.5rem;
                                            flex-shrink: 0;
                                        "/>

                                        // Chat bubble
                                        <div style={format!(
                                            "background-color: {}; color: white; padding: 0.5rem 1rem; border-radius: 1rem; max-width: 70%;",
                                            "#AF69ED"
                                        )}>
                                            { &m.text }
                                        </div>
                                    </>
                                }
                            }}
                        </div>
                    }
                })}
            </div>

            // Input area pinned at bottom
            <div style="
                display: flex;
                padding: 1rem;
                background-color: #1c023dff;
                gap: 0;
                box-sizing: border-box;
            ">
                <input
                    {oninput}
                    {onkeypress}
                    type="text"
                    value={(*input).clone()}
                    placeholder="Aa"
                    style="
                    flex: 1;
                    padding: 0.5rem 1rem;
                    border-radius: 1rem 0 0 1rem;  /* rounded left corners only */
                    border: none;
                    background-color: #2e1b4cff;
                    color: white;
                    outline: none;"
                />
                <button
                    onclick={onclick}
                    style="
                        padding: 0 1rem;
                        border-radius: 0 1rem 1rem 0;  /* rounded right corners only */
                        border: none;
                        background-color: #AF69ED;
                        color: white;
                        cursor: pointer;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    "
                >
                    { "➤" }  // arrow icon inside button
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
