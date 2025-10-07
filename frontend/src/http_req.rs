use yew::prelude::*;
use gloo::net::http::Request;
use crate::types::{ChatMessage,
					Sender, 
					HttpRequest,
					HttpResponse};


pub fn send_message (input: &UseStateHandle<String>, chat_history_onevent: &UseStateHandle<Vec<ChatMessage>>) {
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
        	let request_body = HttpRequest { text: msg.to_string() };
        	let request = Request::post("api/chat")
        		.header("Content-Type", "application/json")
        		.body(serde_json::to_string(&request_body).unwrap())
        		.send();
        	let response = request
        		.await
        		.unwrap();
        	let resp_json: HttpResponse = response.json().await.unwrap();
        	// Add AI reply to chat_history_with_user and add it to chat_history 
        	chat_history_with_curr.push(ChatMessage { sender: Sender::AI, text: resp_json.text.clone() });
        	chat_history_callback.set(chat_history_with_curr);
        });
}