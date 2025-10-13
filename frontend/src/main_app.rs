use yew::prelude::*;
use http_req::send_message;

mod chat_container;
mod input_bar;
mod http_req;
mod types;
mod state_hooks;
mod chat_menu;

#[function_component(App)]
pub fn app() -> Html {
	let input = use_state(|| String::new());
 let chat_history = use_reducer(|| state_hooks::ChatHistory::default());
	// send_message when ons_send is emitted
	let on_send:Callback<()> = { 
		let input_clone = input.clone();
		let chat_history_clone = chat_history.clone();

		Callback::from(move |_| {
			send_message(&input_clone, &chat_history_clone);
		})
	};

	let on_select_chat = {
        Callback::from(move |chat: types::Chat| {
            // here you can fetch messages for the selected chat
            web_sys::console::log_1(&format!("Selected chat: {}", chat.title).into());
        })
    };

    
                        
	html! {
		<>
		 	// Header bar wrapper
        	<div class= "header-div">
        	    <chat_menu::ChatMenu user_id={"user:alice".to_string()} on_select_chat={on_select_chat.clone()} />
        	    <h3 style="font-family: 'Indie Flower', cursive; color: #eee;  margin: 0;">
        	        {"Rikka: Wielder of the Wicked Eye"}
        	    </h3>
			</div>

			 // 💬 Main content
			<chat_container::ChatContainer {chat_history} />
            <input_bar::InputBar {input} {on_send} />           
		</>
	}
}



