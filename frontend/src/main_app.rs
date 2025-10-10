use yew::prelude::*;
use http_req::send_message;

mod chat_container;
mod input_bar;
mod http_req;
mod types;
mod state_hooks;

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
                        
	html! {
		<>
			<h3 style="font-family: 'Indie Flower', cursive; margin: 1rem;"> {"Rikka: Wielder of the Wicked Eye"} </h3>
			<chat_container::ChatContainer {chat_history} />
            <input_bar::InputBar {input} {on_send} />           
		</>
	}
}



