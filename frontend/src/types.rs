// For displaying messages in frontend
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

// For Http requests 
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct HttpRequest {
	pub text: String,
}
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct HttpResponse {
	pub text: String,
}