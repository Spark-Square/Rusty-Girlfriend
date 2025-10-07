use surrealdb::{sql::Thing,
                {RecordId,
                Surreal}};
use chrono::Utc;
use crate::types::{Record,
                    User, 
                    Chat,
                    Sender, 
                    ChatMessage};

// ================== CREATE FUNCTIONS ==================
pub async fn create_user(db: &Surreal<surrealdb::engine::remote::ws::Client>, username: &str, name: &str) -> Option<Record> {
    let user = User {
        username: username.to_string(),
        name: name.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let record: Option<Record> = db.create("user").content(user).await.unwrap();
    record
}

pub async fn create_chat(db: &Surreal<surrealdb::engine::remote::ws::Client>, title: &str, owner: RecordId) -> Option<Record> {
    let chat = Chat {
        title: title.to_string(),
        owner,
        created_at: Utc::now().to_rfc3339(),
    };
    let record: Option<Record> = db.create("chat").content(chat).await.unwrap();
    record
}

pub async fn add_message(db: &Surreal<surrealdb::engine::remote::ws::Client>, chat: RecordId, sender: Sender, text: &str) -> Option<Record> {
    let msg = ChatMessage {
        chat,
        sender,
        text: text.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let record: Option<Record>  = db.create("message").content(msg).await.unwrap();
                      
    record
}

// ================== FETCH FUNCTIONS ==================
pub async fn fetch_messages(db: &Surreal<surrealdb::engine::remote::ws::Client>, chat: Thing) -> Vec<ChatMessage> {
    let messages: Vec<ChatMessage> = db
        .query("SELECT * FROM message WHERE chat = $chat ORDER BY created_at ASC")
        .bind(("chat", chat))
        .await
        .unwrap()
        .take(0)
        .unwrap();
        
        messages
}

pub async fn fetch_user_chats(db: &Surreal<surrealdb::engine::remote::ws::Client>, user: Thing) -> Vec<Chat> {
    let chats: Vec<Chat> = db
        .query("SELECT * FROM chat WHERE owner = $user ORDER BY created_at ASC")
        .bind(("user", user))
        .await
        .unwrap()
        .take(0)
        .unwrap();
        
        chats
}
