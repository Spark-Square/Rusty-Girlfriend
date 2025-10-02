use surrealdb::Surreal;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub title: String,
    pub owner: Thing, // user:xxx
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub chat: Thing,  // chat:xxx
    pub sender: Thing, // user:xxx or Thing::from("ai")
    pub text: String,
    pub created_at: String,
}

// ================== CREATE FUNCTIONS ==================
pub async fn create_user(db: &Surreal<surrealdb::engine::remote::ws::Client>, username: &str, name: &str) -> Result<Thing> {
    let user = User {
        username: username.to_string(),
        name: name.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let id: Thing = db.create("user").content(user).await?    
                      .ok_or_else(|| anyhow::anyhow!("Failed to create user"))?;
    Ok(id)
}

pub async fn create_chat(db: &Surreal<surrealdb::engine::remote::ws::Client>, title: &str, owner: Thing) -> Result<Thing> {
    let chat = Chat {
        title: title.to_string(),
        owner,
        created_at: Utc::now().to_rfc3339(),
    };
    let id: Thing = db.create("chat").content(chat).await?
                      .ok_or_else(|| anyhow::anyhow!("Failed to create chat"))?;
    Ok(id)
}

pub async fn add_message(db: &Surreal<surrealdb::engine::remote::ws::Client>, chat: Thing, sender: Thing, text: &str) -> Result<Thing> {
    let msg = ChatMessage {
        chat,
        sender,
        text: text.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let id: Thing = db.create("message").content(msg).await?
                      .ok_or_else(|| anyhow::anyhow!("Failed to create message"))?;
    Ok(id)
}

// ================== FETCH FUNCTIONS ==================
pub async fn fetch_messages(db: &Surreal<surrealdb::engine::remote::ws::Client>, chat: Thing) -> Result<Vec<ChatMessage>> {
let messages: Vec<ChatMessage> = db
    .query("SELECT * FROM message WHERE chat = $chat ORDER BY created_at ASC")
    .bind(("chat", chat))
    .await?
    .take(0)?;
    Ok(messages)
}

pub async fn fetch_user_chats(db: &Surreal<surrealdb::engine::remote::ws::Client>, user: Thing) -> Result<Vec<Chat>> {
let chats: Vec<Chat> = db
        .query("SELECT * FROM chat WHERE owner = $user ORDER BY created_at ASC")
        .bind(("user", user))
        .await?
        .take(0)?;
        Ok(chats)
}
