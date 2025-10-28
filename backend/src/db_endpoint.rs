use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use rocket::serde::json::Json;

use crate::types::User;

#[get("/db/get_user/<username>" )]
pub async fn db_get_user (db: &State<Surreal<Client>>, username: String) -> Option<Json<User>> {

    // Fetch from SurrealDB
    let result: Option<User> = db
        .select(("user", username)) // table and id
        .await
        .ok()
        ?;
     result.map(Json)

}