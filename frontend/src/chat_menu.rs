use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::types::Chat;
use crate::http_req::get_user_chats;

#[derive(Properties, PartialEq)]
pub struct ChatMenuProps {
    pub user_id: String,
    pub selected_chat: UseStateHandle<Option<Chat>>,
}

#[function_component(ChatMenu)]
pub fn chat_menu(props: &ChatMenuProps) -> Html {

    // Variables of is_open state, chats storage and user_id
    let is_open = use_state(|| false);
    let chats = use_state(|| Vec::<Chat>::new());
    let user_id = props.user_id.clone();
    
    // Callback gotten from parent and to be emitted to when a chat is selected
    let selected_chat = props.selected_chat.clone();


//______________________________________________________________________________________________________________________
    // Manages state of *open* when chatmenu is clicked
    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    // use_effect_with that monitors changes to open and displays chats when open is true
    {
        let is_open = is_open.clone();
        let chats = chats.clone();
        let user_id = user_id.clone();

        use_effect_with(is_open.clone(), move |is_open| {
            if **is_open {
                let chats = chats.clone();
                let user_id = user_id.clone();
                spawn_local(async move {
                    match get_user_chats(&user_id).await {
                        Ok(fetched) => chats.set(fetched),
                        Err(err) => web_sys::console::error_1(&err.into()),
                    }
                });
            }
            || ()
        });
    }

//______________________________________________________________________________________________________________________
    html! {
        <div style="position: relative;">
            // The chats dropdown button
            <button onclick={toggle} class = "chat-menu-button">
                {"☰"}
            </button>

            // Conditional rendering based on *open* state of chatmenu
            { if *is_open {
                html! {
                    <div class="chats-div" >
                        <h4 style="margin: 0.5rem 0; ">{"Your Chats"}</h4>
                        // Unordered lists for chats
                        <ul style="margin: 0;">
//______________________________________________________________________________________________________________________
                            // Iterating through each chat in the use_state vectors of chats
                            {
                                for (*chats).iter().cloned().map(move |chat| {
                                    let selected_chat = selected_chat.clone();
                                    let chat_clicked = chat.clone();

                                    //Callbacks mouse clicking, mouse entering and leaving into each chat title
                                    let onclick={Callback::from(move |_| selected_chat.set(Some(chat_clicked.clone())))};
                                    let onmouseenter={Callback::from(move |e: MouseEvent| {
                                        if let Some(el) = e.target_dyn_into::<HtmlElement>() {
                                            let _ = el.style().set_property("background", "#222");
                                        }
                                    })};
                                    let onmouseleave={Callback::from(move |e: MouseEvent| {
                                        if let Some(el) = e.target_dyn_into::<HtmlElement>() {
                                            let _ = el.style().set_property("background", "transparent");
                                        }
                                    })};

                                    //Finally the actual chats, gosh this is messy
                                    html! {
                                        <li 
                                            class= "chat-title-menu"
                                            {onclick}
                                            {onmouseenter}
                                            {onmouseleave}
                                        >
                                            { &chat.title }
                                        </li>
                                    }
                                })
                            }
//______________________________________________________________________________________________________________________
                        </ul>

                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
