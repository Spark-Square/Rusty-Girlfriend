// For displaying messages in frontend
#[derive(Clone, PartialEq)]
pub struct ChatMessage{
        pub sender: Sender,
        pub text: String,
}
#[derive(Serialize, Deserialize, Debug,Clone,PartialEq)]
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

// Database function types
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub name: String,
    pub created_at: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Chat {
    pub title: String,
    pub owner: String, 
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBChatMessage {
    pub chat: String,
    pub sender: Sender,
    pub text: String,
    pub created_at: String,
}
