use rocket::State;
use std::error::Error;
use rocket::response::Debug;
use crate::mongo::Mongo;
use serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use rocket::serde::json::Json;
use crate::book::Book;

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct Genre {
    pub name: String,
}

#[rocket::post("/api/genre", data = "<genre>")]
pub async fn create_genre(genre: Json<Genre>, db: &State<Mongo>) -> Result<Json<Genre>, Debug<Box<dyn Error>>> {
    let new_genre = db.create_genre(genre.into_inner()).await?;
    Ok(Json(new_genre))
}

#[rocket::get("/api/genre")]
pub async fn get_genres(db: &State<Mongo>) -> Result<Json<Vec<Genre>>, Debug<Box<dyn Error>>> {
    let genres = db.get_all_genres().await?;
    Ok(Json(genres))
}

// list all books by gender name
#[rocket::get("/api/genre/<name>")]
pub async fn get_books_by_genre(name: &str, db: &State<Mongo>) -> Result<Json<Vec<Book>>, Debug<Box<dyn Error>>> {
    let books = db.get_books_by_genre(name).await?;
    Ok(Json(books))
}

