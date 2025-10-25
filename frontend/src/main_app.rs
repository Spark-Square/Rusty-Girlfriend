use yew::prelude::*;
use http_req::send_message;

mod chat_container;
mod input_bar;
mod http_req;
mod types;
mod state_hooks;
mod chat_menu;

use types::Chat;

#[function_component(App)]
pub fn app() -> Html {
 	let chat_history = use_reducer(|| state_hooks::ChatHistory::default());
	let selected_chat: UseStateHandle<Option<Chat>> = use_state(|| None); 
	
	// send_message when ons_send is emitted
	let on_send:Callback<std::string::String> = { 
		let chat_history_clone = chat_history.clone();

		Callback::from(move |input: String| {
			send_message(input, &chat_history_clone);
		})
	};
	// Add use_effect_with to react to selected_chat changes
    use_effect_with(selected_chat.clone(), move |chat_opt| {
        if let Some(chat) = chat_opt.as_ref() {
            web_sys::console::log_1(&format!("Effect: Selected chat {}", chat.title).into());
            // fetch chat messages of the chat here
			// and add them to chat history
        }
    });
              
	html! {
		<>
		 	// Header bar wrapper
        	<div class= "header-div">
        	    <chat_menu::ChatMenu user_id={"user:alice".to_string()} selected_chat={selected_chat.clone()} />
        	    <h3 style="font-family: 'Indie Flower', cursive; color: #eee;  margin: 0;">
        	        {"Rikka: Wielder of the Wicked Eye"}
        	    </h3>
			</div>

			 // 💬 Main content
			<chat_container::ChatContainer {chat_history} />
            <input_bar::InputBar {on_send} />           
		</>
	}
}



