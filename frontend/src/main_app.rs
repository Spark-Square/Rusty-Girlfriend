mod chat_container;
mod input_bar;
mod http_req;

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ChatMessage{
        pub sender: Sender,
        pub text: String,
}
#[derive(Clone,PartialEq)]
pub enum Sender {
        User,
	AI,
}

#[function_component(App)]
pub fn app() -> Html {
	let input = use_state(|| "".to_string());
	let chat_history = use_state(|| vec![]);
                        

	html! {
		//Main div
		<div class = "main-div">
			<h3 style="font-family: 'Indie Flower', cursive; margin: 1rem;"> {"Rikka: Wielder of the Wicked Eye"} </h3>
			<chat_container::ChatContainer chat_history={(*chat_history).clone()} />
            <input_bar::InputBar input= {input.clone()} chat_history= {chat_history.clone()} />           
		</div>
	}
}

