use rocket::serde::{Serialize, Deserialize};

//Custom request and response structs
#[derive(Serialize, Deserialize)]
pub struct HttpRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub text: String,
}

// Async submit response from AI Horde
#[derive(Deserialize, Serialize, Debug)]
pub struct AIHordeSubmitResponse {
    pub id: String,
    pub done: Option<bool>,  //  make it optional
    pub message: Option<String>,
    pub kudos: Option<f64>,
}


// Async status response from AI Horde
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Generation {
    pub text: Option<String>, // sometimes missing, so make it Option
    pub seed: Option<u64>,
    pub gen_metadata: Option<Vec<GenMetadata>>,
    pub worker_id: Option<String>,
    pub worker_name: Option<String>,
    pub model: Option<String>,
    pub state: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GenMetadata {
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub ref_: Option<String>, // if field is literally "ref" in JSON, need #[serde(rename="ref")]
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AIHordeStatusResponse {
    pub generations: Option<Vec<Generation>>,
    pub done: Option<bool>,
    pub processing: Option<u64>,
    pub waiting: Option<u64>,
    pub wait_time: Option<u64>,
    pub queue_position: Option<u64>,
    pub is_possible: Option<bool>,
    pub finished: Option<u64>,
    pub restarted: Option<u64>,
    pub faulted: Option<bool>,
    pub kudos: Option<f64>,
}

use surrealdb::RecordId;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub  id: RecordId,
}


// Database function types 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub title: String,
    pub owner: RecordId, // user:xxx
    pub created_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum  Sender {
    User, 
    AI
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub chat: RecordId,  // chat:xxx
    pub sender: Sender,
    pub text: String,
    pub created_at: String,
}
