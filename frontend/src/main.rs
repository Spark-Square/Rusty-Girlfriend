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

//--------------------------------------------------------------------------------------------------------------------------------
#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(|| "".to_string());
    let chat_history = use_state(|| vec![]);

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let input_elem = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            input.set(input_elem.value());
        })
    };
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
                if e.shift_key() {  // Shift+Enter -> newline
                    // insert a newline at the current cursor position
                    if let Some(input_elem) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                        let cursor_pos = input_elem.selection_start()
                                                          .unwrap()
                                                          .unwrap();                     
                        let mut new_value = (*input).clone();
                        new_value.insert_str(cursor_pos as usize, "\n");
                        input.set(new_value);
                        // move cursor forward by 1
                        input_elem.set_selection_start(Some(cursor_pos + 1)).ok();
                        input_elem.set_selection_end(Some(cursor_pos + 1)).ok();
                    }
                } else {  // Enter -> send message
                    e.prevent_default();
                    send_message(&input, &chat_history_onkeypress); // reuse your send_message function
                }
            }
        })
    };
//--------------------------------------------------------------------------------------------------------------------------------                        
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
//--------------------------------------------------------------------------------------------------------------------------------                        
    // Auto-scroll when chat_history updates   
    let chat_container_ref = use_node_ref().clone();    // NodeRef for chat container

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
//--------------------------------------------------------------------------------------------------------------------------------                        
    html! {
        //Main div
        <div class = "main-div">
            <h3 style="margin: 1rem;">{ "Rikka: Wielder of the Wicked Eye" }</h3>
//--------------------------------------------------------------------------------------------------------------------------------                        
            // Chat container div
            <div class= "chat-container-div" ref = {chat_container_ref} > // attach NodeRef
                { for chat_history.iter().map(|m| {
                    let is_user = m.sender == Sender::User;
//--------------------------------------------------------------------------------------------------------------------------------
                    
                    html! {
                        //Message and icon div
                        <div class={classes!("message_and_icon_div")} 
                        style = {format!( "flex-direction: {};", if is_user {  "row-reverse" } else { "row" })}>
                            // Render bubble first if user, avatar second
                            {
                                html! {
                                    <>
                                        // Avatar icon
                                        {if is_user { html!(<img src="/Icons/Yuuta.jpg" alt= "Me"/>)}
                                        else { html!{<img src="/Icons/Rikka.jpg" alt= "Ai"/>}}}
                                        
                                        // Chat bubble
                                        <div class ="chat_bubble" 
                                        style={format!("background-color: {}; ", if is_user {"#007bff; margin-right: 0.55rem"} 
                                                                                 else {"#AF69ED; margin-left: 0.55rem"}
                                        )}>
                                            { &m.text }
                                        </div>
                                    </>
                                }
                            }
                        </div>
                    }
//--------------------------------------------------------------------------------------------------------------------------------                        
                })}
            </div>
//--------------------------------------------------------------------------------------------------------------------------------                        
            // Input area pinned at bottom
            <div class = "input-bar-area">
                <textarea
                    {oninput}
                    {onkeypress}
                    type="text"
                    value={(*input).clone()}
                    placeholder="Aa"
                    class= "msg-input"
                />
                <button onclick={onclick} class = "send-button">
                    { "➤" }  // arrow icon inside button
                </button>
            </div>
//--------------------------------------------------------------------------------------------------------------------------------                        
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
