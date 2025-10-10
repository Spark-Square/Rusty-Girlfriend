use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::{Function, Object, Reflect};

use crate::state_hooks;
use crate::types::{Sender};

#[derive(Properties, PartialEq)]
pub struct ChatContainerProperties {
	pub chat_history: UseReducerHandle<state_hooks::ChatHistory>,
}

#[function_component(ChatContainer)]
pub fn chat_container (props: &ChatContainerProperties) -> Html {

	// Auto-scroll when chat_history updates   
	let chat_container_ref = use_node_ref().clone();    // NodeRef for chat container

	{	let chat_container_ref_clone = chat_container_ref.clone(); // make a clone for the closure
		use_effect_with(
		    props.chat_history.clone(),
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
	}

	html!{
	        // Chat container div
            	<div class= "chat-container-div" ref = {chat_container_ref}> // attach NodeRef
                	{ for props.chat_history.messages.iter().map(|m| {
                    		let is_user = m.sender == Sender::User;
				
				html! {
                        		// Chat row div, common stlying and unique styling
                        		<div class={classes!( "chat_row_div", {format!( "{}", if is_user {  "user_chat_row_div" } else { "ai_chat_row_div" })} )}>
                            		// Icon -> Message as default (AI), then reverse it for user
                            			{
                                			html! {
                                    				<>
                                        				// Avatar icon
                                        				{if is_user { html!(<img src="/Icons/Yuuta.jpg" alt= "Me"/>)}
                                        				else { html!{<img src="/Icons/Rikka.jpg" alt= "Ai"/>}}}
                                        
                                        				// Chat bubble
                                        				<div class = { classes! ("chat_bubble", {format!( "{}", if is_user {"user_chat_bubble"} else {"ai_chat_bubble"})} )}>
                                        				    { &m.text }
                                        				</div>
                                    				</>
                                			}
                            			}
                        		</div>
				}
			})}
		</div>
	}
}

