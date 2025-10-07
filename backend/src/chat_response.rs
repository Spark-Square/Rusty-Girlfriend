use rocket::serde::{Serialize, Deserialize, json::Json};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

use crate::db_functions::{add_message, create_chat, create_user, Sender};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws;
use surrealdb::opt::auth;
use surrealdb::RecordId;

// ----------------- DATA STRUCTS -----------------
    
//Custom request and response structs
#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    reply: String,
}

// Async submit response from AI Horde
#[derive(Deserialize, Serialize, Debug)]
struct AIHordeSubmitResponse {
    id: String,
    done: Option<bool>,  //  make it optional
    message: Option<String>,
    kudos: Option<f64>,
}


// Async status response from AI Horde
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Generation {
    text: Option<String>, // sometimes missing, so make it Option
    seed: Option<u64>,
    gen_metadata: Option<Vec<GenMetadata>>,
    worker_id: Option<String>,
    worker_name: Option<String>,
    model: Option<String>,
    state: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GenMetadata {
    r#type: Option<String>,
    value: Option<String>,
    ref_: Option<String>, // if field is literally "ref" in JSON, need #[serde(rename="ref")]
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AIHordeStatusResponse {
    generations: Option<Vec<Generation>>,
    done: Option<bool>,
    processing: Option<u64>,
    waiting: Option<u64>,
    wait_time: Option<u64>,
    queue_position: Option<u64>,
    is_possible: Option<bool>,
    finished: Option<u64>,
    restarted: Option<u64>,
    faulted: Option<bool>,
    kudos: Option<f64>,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub  id: RecordId,
}

// ----------------- CHAT ENDPOINT -----------------

#[post("/api/chat", format = "json", data = "<chat>")]
pub async fn chat_response(chat: Json<ChatRequest>) -> Json<ChatResponse> {
    
let db = match Surreal::new::<ws::Ws>("127.0.0.1:8001").await {
    Ok(db) => db,
    Err(e) => {
        eprintln!("DB connection failed: {:?}", e);
        return Json(    // 1️⃣ Connect to SurrealDB
            ChatResponse {
                reply: "Internal error: cannot connect to database".to_string(),
            });
    }
};    
    
    db.signin(auth::Root {
        username: "root",
        password: "root",
    }).await.unwrap();
    db.use_ns("test").use_db("chat_db").await.unwrap();


async fn setup_mock_user_chat(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> Record {
    // Create a mock user
    let user_record:Record = create_user(db, "alice", "Alice").await.unwrap();
    // Create a chat for that user
    let chat_record:Record= create_chat(db, "Rikka Chat", user_record.id.clone()).await.unwrap();

    chat_record
    
}


    // 2️⃣ Setup mock user/chat (in real app, use logged-in user)
    let chat_rec = setup_mock_user_chat(&db).await;

    // 3️⃣ Add user message to DB
    let _user_msg_id = add_message(&db, chat_rec.id.clone(), Sender::User, &chat.message).await.unwrap();

    // load environment variables and get api key
    dotenvy::dotenv().ok();
    let api_key = std::env::var("AI_HORDE_API_KEY")
        .expect("AI_HORDE_API_KEY must be present in backend/.env");

    // Set https request contents in variables, and payload in json
    let api_url = "https://stablehorde.net/api/v2/generate/text/async";
    let client = Client::new();
    let payload = serde_json::json!({
        "prompt": chat.message,
        //"models": ["Erebus","Shinen"]
    });

    // ---- submit request ----
    let submit_send = client
        .post(api_url)
        .header("apikey", api_key.clone())
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    let submit_resp = match submit_send {
        Err(e) => {
            return Json(ChatResponse {
                reply: format!("Error sending submit request: {}\nPayload: {}\n", e, payload),
            });
        }
        Ok(r) => r,
    };

    // Read raw text of submit response
    let submit_text = match submit_resp.text().await {
        Ok(t) => t,
        Err(e) => {
            return Json(ChatResponse {
                reply: format!("Error reading submit response text: {}\n(Unable to read body)", e),
            });
        }
    };

    // Try parsing submit JSON
    let mut submit_parsed: AIHordeSubmitResponse = match serde_json::from_str(&submit_text) {
        Ok(parsed) => parsed,
        Err(e) => {
            return Json(ChatResponse {
                reply: format!(
                    "AI Horde returned invalid JSON on submit: {}\nRaw response: {}",
                    e, submit_text
                ),
            });
        }
    };

    // Ensure we have an id to poll
    let job_id = if submit_parsed.id.is_empty() {
        // No id -> return the raw JSON so user can see what's up 
        eprintln!("Error: Job ID missing in response: {:?}", submit_parsed);
        return Json(ChatResponse {
                reply: format!("AI Horde submit response missing job id.\nRaw response: {}", submit_text),
            });
    } else {
        submit_parsed.id.clone()
    };


    // ---- polling loop (for style: for loop) ----
    let status_url_base = "https://stablehorde.net/api/v2/generate/text/status";
    let poll_interval = Duration::from_secs(2);
    let max_attempts = 200;

    // ------------------------------ DEBUG ------------------------------// 
    // We'll keep the last raw poll text to show on final failure
    //let mut last_poll_raw = String::new();

    for attempt in 0..max_attempts {

        if attempt != 0 {
            sleep(poll_interval).await;
        }


        let poll_url = format!("{}/{}", status_url_base, job_id);
        // send poll request
        let poll_send = client
            .get(&poll_url)
            .header("apikey", api_key.clone())
            .send()
            .await;

        let poll_resp = match poll_send {
            Err(e) => {
                return Json(ChatResponse {
                    reply: format!("Error sending poll request (attempt {}): {}\nPoll URL: {}", attempt, e, poll_url),
                });
            }
            Ok(r) => r,
        };

        // read raw poll text
        let poll_text = match poll_resp.text().await {
            Ok(t) => t,
            Err(e) => {
                return Json(ChatResponse {
                    reply: format!("Error reading poll response text (attempt {}): {}\nPoll URL: {}", attempt, e, poll_url),
                });
            }
        };

        // ------------------------------ DEBUG ------------------------------// 
        // print full raw JSON per poll for inspection 
        //println!("Poll attempt {}: raw JSON:\n{}", attempt + 1, poll_text);
        // save last raw JSON
        //last_poll_raw = poll_text.clone();

        // try parse poll JSON
        let parsed_status: AIHordeStatusResponse = match serde_json::from_str(&poll_text) {
            Ok(p) => p,
            Err(e) => {
                return Json(ChatResponse {
                    reply: format!("AI Horde returned invalid JSON while polling (attempt {}): {}\nRaw response: {}", attempt, e, poll_text),
                });
            }
        };


        // break if the done= true
        if parsed_status.done == Some(true) {        
            if let Some(generations) = parsed_status.generations {
                if let Some(first) = generations.first() {
                    if let Some(text) = &first.text {
                        
                        // reply in DB
                        let _ai_msg_id = {
                            add_message(&db, chat_rec.id.
                                clone(), Sender::AI, &text).await.unwrap()                        };
                        return Json(ChatResponse {
                            reply: text.clone(),
                        });
                    }
                }
            }

            break;
        } 

        // if this iteration is the last and still not done -> return last raw JSON
        if attempt == max_attempts && submit_parsed.done != Some(true) {
            return Json(ChatResponse {
                reply: format!("AI Horde did not finish after {} attempts. Last poll JSON:\n{}", max_attempts, poll_text),
            });
        }

        // update submit_parsed.done/message so the loop condition can use it
        submit_parsed.done = parsed_status.done;

    }
    // ------------------------------ DEBUG ------------------------------// 
    // At this point either done == true or we broke/returned early
    // DEBUG: print full raw JSON for inspection
    //println!("Last Poll raw JSON:\n{}", last_poll_raw);
    // fallback: no message even though done may be true

    Json(ChatResponse { reply: "Error: no prompt answer to respond with; Either our code failed or AIHorde did not respond".to_string() })
}
