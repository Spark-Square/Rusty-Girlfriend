use surrealdb::{Surreal,
                engine::remote::ws::{Ws,Client},
                opt::auth,
                sql::Thing,RecordId};
use chrono::Utc;
use crate::types::{Record,
                    User, 
                    Chat,
                    Sender, 
                    ChatMessage};

#[allow(dead_code)]
pub struct  Database {
    pub client: Surreal<Client>,
    pub ip: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl Database {
    pub async fn init(
        ip: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
        namespace: impl Into<String>,
        database: impl Into<String>,
    ) -> Self {
        let ip = ip.into();
        let username = username.into();
        let password = password.into();
        let namespace = namespace.into();
        let database = database.into();

        // Connect
        let client = Surreal::new::<Ws>(&ip).await.unwrap();

        // Sign in
        client.signin(auth::Root {
            username: &username,
            password: &password,
        }).await.unwrap();

        // Select namespace and database
        client.use_ns(&namespace).use_db(&database).await.unwrap();

        Self {
            client,
            ip,
            username,
            password,
            namespace,
            database,
        }
    }
}

// ================== CREATE FUNCTIONS ==================
pub async fn create_user(db: &Surreal<Client>, username: &str, name: &str) -> Option<Record> {
    let user = User {
        username: username.to_string(),
        name: name.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    let record: Option<Record> = db.create(("user", username)).content(user).await.unwrap();
    record
}

pub async fn ensure_user(
    db: &Surreal<Client>,
    username: &str,
    name: &str,
) -> Option<Record> {
    // Try to select the record by its ID
    if let Ok(Some(existing)) = db.select(("user", username)).await {
        return existing;
    }

    // If not found, create it
    create_user(db, username, name).await
}

pub async fn create_chat(db: &Surreal<Client>, title: &str, owner: RecordId) -> Option<Record> {
    let chat = Chat {
        title: title.to_string(),
        owner,
        created_at: Utc::now().to_rfc3339(),
    };

    match  db.create("chat").content(chat).await {
       Ok(id) => id,
       Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            None
       },
    }
}

pub async fn add_message(db: &Surreal<Client>, chat: RecordId, sender: Sender, text: &str) -> Option<Record> {
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



pub async fn fetch_user_chats(db: &Surreal<Client>, user: Thing) -> Vec<Chat> {
    let chats: Vec<Chat> = db
        .query("SELECT * FROM chat WHERE owner = $user ORDER BY created_at ASC")
        .bind(("user", user))
        .await
        .unwrap()
        .take(0)
        .unwrap();
        
        chats
}

pub async fn fetch_messages(db: &Surreal<Client>, chat: Thing) -> Vec<ChatMessage> {
    let messages: Vec<ChatMessage> = db
        .query("SELECT * FROM message WHERE chat = $chat ORDER BY created_at ASC")
        .bind(("chat", chat))
        .await
        .unwrap()
        .take(0)
        .unwrap();
        
        messages
}