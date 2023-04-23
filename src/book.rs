use std::collections::HashMap;
use rocket::State;
use std::error::Error;
use rocket::response::Debug;
use crate::mongo::Mongo;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use crate::user::User;
use crate::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
    pub resume: String,
    pub availability: bool,
    pub gender_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<i32>,
    pub resume: Option<String>,
    pub availability: Option<bool>,
    pub gender_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: i32,
    pub resume: String
}

impl From<NewBook> for Book {
    fn from(value: NewBook) -> Self {
        Book {
            title: value.title,
            author: value.author,
            year: value.year,
            resume: value.resume,
            availability: true,
            gender_id: "000000000000000000000000".to_string(),
        }
    }
}

#[rocket::post("/api/book", data = "<book>")]
pub async fn create_book(book: Json<NewBook>, db: &State<Mongo>) -> Result<Json<Book>, Debug<Box<dyn Error>>> {
    let new_book = db.create_book(book.into_inner()).await?;
    Ok(Json(new_book))
}

#[rocket::get("/api/book")]
pub async fn get_books(db: &State<Mongo>) -> Result<Json<Vec<Book>>, Debug<Box<dyn Error>>> {
    let books = db.get_all_books().await?;
    Ok(Json(books))
}

#[rocket::get("/api/book/<id>")]
pub async fn get_book(id: &str, db: &State<Mongo>) -> Result<Json<Book>, Debug<Box<dyn Error>>> {
    let book = db.get_book_by_id(id).await?;
    Ok(Json(book))
}

#[rocket::put("/api/book/<id>", data = "<book>")]
pub async fn update_book(id: &str, book: Json<UpdateBook>, db: &State<Mongo>) -> Result<Json<Book>, Debug<Box<dyn Error>>> {
    let mut hashmap = HashMap::new();

    if book.title.is_none() && book.author.is_none() && book.year.is_none() && book.gender_id.is_none() && book.resume.is_none() && book.availability.is_none() {
        return Ok(Json(db.get_book_by_id(id).await?));
    }
    match &book.title {
        Some(title) => hashmap.insert("title", Value::Text(title.clone())),
        None => None,
    };
    match &book.author {
        Some(author) => hashmap.insert("author", Value::Text(author.clone())),
        None => None,
    };
    match &book.year {
        Some(year) => hashmap.insert("year", Value::Int(*year)),
        None => None,
    };
    match &book.resume {
        Some(resume) => hashmap.insert("resume", Value::Text(resume.clone())),
        None => None,
    };
    match &book.gender_id {
        Some(gender_id) => hashmap.insert("gender_id", Value::Text(gender_id.clone())),
        None => None,
    };
    match &book.availability {
        Some(availability) => hashmap.insert("availability", Value::Bool(*availability)),
        None => None,
    };
    let updated_book = db.update_book(id, hashmap).await?;
    Ok(Json(updated_book))
}

// delete book
#[rocket::delete("/api/book/<id>")]
pub async fn delete_book(id: &str, db: &State<Mongo>) -> Result<Json<Book>, Debug<Box<dyn Error>>> {
    let deleted_book = db.delete_book(id).await?;
    Ok(Json(deleted_book))
}

// search book
#[rocket::post("/api/book/search", data = "<book>")]
pub async fn search_book(book: Json<SearchBook>, db: &State<Mongo>) -> Result<Json<Vec<Book>>, Debug<Box<dyn Error>>> {

    let mut hashmap = HashMap::new();
    if book.title.is_none() && book.author.is_none() && book.year.is_none() {
        return Ok(Json(vec![]));
    }
    match &book.title {
        Some(title) => hashmap.insert("title", title.clone()),
        None => None
    };
    match &book.author {
        Some(author) => hashmap.insert("author", author.clone()),
        None => None
    };
    match &book.year {
        Some(year) => hashmap.insert("year", year.to_string()),
        None => None
    };

    let books = db.search_book(hashmap).await?;
    Ok(Json(books))
}

// borrow book
#[rocket::post("/api/book/<id>/<user_id>/borrow")]
pub async fn borrow_book(id: &str, user_id: &str, db: &State<Mongo>) -> Result<Json<(User, Book)>, Debug<Box<dyn Error>>> {
    let borrowed_book = db.borrow_book(id, user_id).await?;
    Ok(Json(borrowed_book))
}

// return book
#[rocket::post("/api/book/<id>/<user_id>/return")]
pub async fn return_book(id: &str, user_id: &str, db: &State<Mongo>) -> Result<Json<(User, Book)>, Debug<Box<dyn Error>>> {
    let returned_book = db.return_book(id, user_id).await?;
    Ok(Json(returned_book))
}
