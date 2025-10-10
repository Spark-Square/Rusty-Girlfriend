use yew::prelude::*;
use gloo::net::http::Request;
use crate::{state_hooks::{ChatHistory,
						ChatAction}, 
			types::{HttpRequest, 
					HttpResponse, 
					Sender}};


pub fn send_message (input: &UseStateHandle<String>, chat_history: &UseReducerHandle<ChatHistory>) {
        let msg = (*input).clone();

        if msg.is_empty() {
        	return;
        }

        // Immediately add the user message via reducer
    	chat_history.dispatch(ChatAction::AddMessage {
        		sender: Sender::User,
        		text: msg.to_string(),
    	});
    	// clear input
    	input.set(String::new());


    	// Move clones into the async task
    	let chat_history_for_async = chat_history.clone();
        
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
			
			chat_history_for_async.dispatch(ChatAction::AddMessage {
            	sender: Sender::AI,
            	text: resp_json.text,
        	});
        	
        });
}