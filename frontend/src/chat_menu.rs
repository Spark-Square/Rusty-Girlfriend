use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::types::Chat;
use crate::http_req::get_user_chats;

#[derive(Properties, PartialEq)]
pub struct ChatMenuProps {
    pub user_id: String,
    pub on_select_chat: Callback<Chat>,
}

#[function_component(ChatMenu)]
pub fn chat_menu(props: &ChatMenuProps) -> Html {
    let open = use_state(|| false);
    let chats = use_state(|| Vec::<Chat>::new());

    let user_id = props.user_id.clone();
    let on_select_chat = props.on_select_chat.clone();

    // Fetch chats when dropdown opens
    {
        let open = open.clone();
        let chats = chats.clone();
        let user_id = user_id.clone();

        use_effect_with(open.clone(), move |is_open| {
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

    // Toggle dropdown
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    html! {
        <div style="position: relative;">
            // Toggle button
            <button
                onclick={toggle}
                style="
                    position: absolute;
                    top: 1rem;
                    left: 1rem;
                    background-color: #2b2b2b;
                    color: white;
                    border: none;
                    border-radius: 8px;
                    padding: 0.5rem 0.9rem;
                    cursor: pointer;
                    z-index: 1100;
                "
            >
                {"☰ Chats"}
            </button>

            { if *open {
                html! {
                    <div
                        style="
                            position: fixed;
                            top: 3.6rem;
                            left: 0;
                            width: 260px;
                            max-height: 80vh;
                            overflow-y: auto;
                            background: #111;
                            color: #eee;
                            padding: 0.5rem;
                            box-shadow: 0 6px 18px rgba(0,0,0,0.5);
                            border-top-right-radius: 12px;
                            z-index: 1050;
                        "
                    >
                        <h4 style="margin: 0.25rem 0 0.5rem 0; padding: 0 0.25rem;">{"Your Chats"}</h4>
                        <ul style="list-style: none; margin: 0; padding: 0;">
                            {
                                for (*chats).iter().cloned().map(move |chat| {
                                    let on_select_chat = on_select_chat.clone();
                                    let chat_for_click = chat.clone();

                                    html! {
                                        <li
                                            onclick={Callback::from(move |_| on_select_chat.emit(chat_for_click.clone()))}
                                            onmouseenter={Callback::from(move |e: MouseEvent| {
                                                if let Some(el) = e.target_dyn_into::<HtmlElement>() {
                                                    let _ = el.style().set_property("background", "#222");
                                                }
                                            })}
                                            onmouseleave={Callback::from(move |e: MouseEvent| {
                                                if let Some(el) = e.target_dyn_into::<HtmlElement>() {
                                                    let _ = el.style().set_property("background", "transparent");
                                                }
                                            })}
                                            style="
                                                padding: 0.55rem 0.6rem;
                                                cursor: pointer;
                                                border-radius: 6px;
                                                margin-bottom: 0.15rem;
                                                transition: background 0.12s ease;
                                                user-select: none;
                                            "
                                        >
                                            { &chat.title }
                                        </li>
                                    }
                                })
                            }
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
