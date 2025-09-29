use yew::prelude::*;

use crate::chat_container::ChatMessage;
use crate::http_req::send_message;

#[derive(Properties, PartialEq)]
pub struct InputBarProperties{
      pub input: UseStateHandle<String>,
      pub chat_history: UseStateHandle<Vec<ChatMessage>>  
}

#[function_component(InputBar)]
pub fn input_bar (props: &InputBarProperties) -> Html {

        let oninput = {
		let input = props.input.clone();
		Callback::from(move |e: InputEvent| {
			let input_elem = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
			input.set(input_elem.value());
		})
	};
	let on_send_click = {
		let input: UseStateHandle<String> = props.input.clone();
		let chat_history: UseStateHandle<Vec<ChatMessage>> = props.chat_history.clone();
		Callback::from(move |_: MouseEvent| {
			send_message(&input, &chat_history);
		})
		
	};
		// Enter to send message
	let onkeypress = {
		let input = props.input.clone();
		let chat_history_onkeypress = props.chat_history.clone();
		Callback::from(move |e: KeyboardEvent| {
			if e.key() == "Enter" {
				if e.shift_key() {  // Shift+Enter -> newline
					// insert a newline at the current cursor position
					if let Some(input_elem) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
						let cursor_pos = input_elem.selection_start().unwrap().unwrap();

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





        html!{
                // Input area pinned at bottom
		<div class = "input-bar-area">
			<textarea
				{oninput}
				{onkeypress}
				type="text"
				value={(*props.input).clone()}
				placeholder="Aa"
				class= "chat-input"
			/>
			<button onclick={on_send_click} class = "send-button">
				{ "➤" }  // arrow icon inside button
			</button>
		</div>
        }
}