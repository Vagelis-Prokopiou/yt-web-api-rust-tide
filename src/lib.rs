use tide::Body;

use crate::models::User;

pub mod models;

pub async fn get_users() -> tide::http::Result<Body> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            Id: index,
            Age: 25,
            First_Name: format!("First_Name{}", index),
            Last_Name: format!("Last_Name{}", index),
            Framework: "Rust (Axum)".to_owned(),
        })
    }
    Body::from_json(&users)
}
