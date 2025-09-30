use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputBarProperties{
		pub input: UseStateHandle<String>,
    	pub on_send: Callback<()>  
}

#[function_component(InputBar)]
pub fn input_bar (props: &InputBarProperties) -> Html {

        let oninput = {
		let input: UseStateHandle<String> = props.input.clone();
		Callback::from(move |e: InputEvent| {
			let input_elem = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
			input.set(input_elem.value());
		})
	};
	let on_send_click = {
		let on_send = props.on_send.clone();
		Callback::from(move |_| {
			on_send.emit(());
		})
	};
	// Enter to send message
	let onkeypress: Callback<KeyboardEvent> = {
		let on_send = props.on_send.clone();
		let input: UseStateHandle<String> = props.input.clone();

		Callback::from(move |event: KeyboardEvent| {
			if event.key() == "Enter" {
				if event.shift_key() {  // Shift+Enter -> newline
					// insert a newline at the current cursor position
					if let Some(input_elem) = event.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
						let cursor_pos = input_elem.selection_start().unwrap().unwrap();

						let mut new_value = (*input).clone();
						new_value.insert_str(cursor_pos as usize, "\n");
						input.set(new_value);
						// move cursor forward by 1
						input_elem.set_selection_start(Some(cursor_pos + 1)).ok();
						input_elem.set_selection_end(Some(cursor_pos + 1)).ok();
					}
				} else {  // Enter -> send message
					event.prevent_default();
					on_send.emit(());
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