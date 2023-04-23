use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use rocket::serde::json::Json;
use rocket::State;
use crate::mongo::Mongo;
use std::error::Error;
use rocket::response::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_date: String,
    pub borrowed_books: Vec<String>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct SearchUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birth_date: Option<String>,
    pub borrowed_books: Option<Vec<String>>,
    pub role: Option<String>,
}

impl From<NewUser> for User {
    fn from(value: NewUser) -> Self {
        User {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            birth_date: value.birth_date,
            borrowed_books: Vec::new(),
            role: "user".to_string(),
        }
    }
}

#[rocket::post("/api/user", data = "<user>")]
pub async fn create_user(user: Json<NewUser>, db: &State<Mongo>) -> Result<Json<User>, Debug<Box<dyn Error>>> {
    let new_user = db.create_user(user.into_inner()).await?;
    Ok(Json(new_user))
}

#[rocket::get("/api/user")]
pub async fn get_users(db: &State<Mongo>) -> Result<Json<Vec<User>>, Debug<Box<dyn Error>>> {
    let users = db.get_all_users().await?;
    Ok(Json(users))
}

#[rocket::delete("/api/user/<id>")]
pub async fn delete_user(id: &str, db: &State<Mongo>) -> Result<Json<User>, Debug<Box<dyn Error>>> {
    let user = db.delete_user(id).await?;
    Ok(Json(user))
}

#[rocket::post("/api/user/search", data = "<user>")]
pub async fn search_user(user: Json<SearchUser>, db: &State<Mongo>) -> Result<Json<Vec<User>>, Debug<Box<dyn Error>>> {

    let mut hashmap = HashMap::new();
    if user.first_name.is_none() && user.last_name.is_none() && user.email.is_none() {
        return Err(Debug(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No search criteria provided"))));
    }
    match &user.first_name {
        Some(first_name) => hashmap.insert("first_name", first_name.clone()),
        None => None,
    };
    match &user.last_name {
        Some(last_name) => hashmap.insert("last_name", last_name.clone()),
        None => None,
    };
    match &user.email {
        Some(email) => hashmap.insert("email", email.clone()),
        None => None,
    };
    let users = db.search_user(hashmap).await?;
    Ok(Json(users))
}

#[rocket::put("/api/user/<id>", data = "<user>")]
pub async fn update_user(id: &str, user: Json<UpdateUser>, db: &State<Mongo>) -> Result<Json<User>, Debug<Box<dyn Error>>> {
    let mut hashmap = HashMap::new();

    match &user.first_name {
        Some(first_name) => hashmap.insert("first_name", first_name.clone()),
        None => None,
    };
    match &user.last_name {
        Some(last_name) => hashmap.insert("last_name", last_name.clone()),
        None => None,
    };
    match &user.email {
        Some(email) => hashmap.insert("email", email.clone()),
        None => None,
    };
    match &user.birth_date {
        Some(birth_date) => hashmap.insert("birth_date", birth_date.clone()),
        None => None,
    };
    if user.borrowed_books.is_some() {
        for book in user.borrowed_books.clone().unwrap() {
            hashmap.insert("borrowed_books", book.clone());
        }
    };
    match &user.role {
        Some(role) => hashmap.insert("role", role.clone()),
        None => None,
    };

    let updated_user = db.update_user(id, hashmap).await?;
    Ok(Json(updated_user))
}