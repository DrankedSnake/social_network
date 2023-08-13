use std::collections::HashMap;
use actix_web::{Responder, HttpRequest, web::Json, Result};


pub async fn get_users(request: HttpRequest) -> Result<impl Responder>{
    let mut user: HashMap<&str, &str> = HashMap::new();
    user.insert("first_name", "John");
    user.insert("last_name", "Doe");
    user.insert("age", "32");

    let users = vec![user];

    Ok(Json(users))
}