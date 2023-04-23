use rocket::State;
use std::error::Error;
use rocket::response::Debug;
use crate::mongo::Mongo;
use serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use rocket::serde::json::Json;
use crate::OperatorRating;
use crate::book::Book;

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct Comment {
    pub user_id: String,
    pub book_id: String,
    pub comment: String,
    pub rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct UpdateComment {
    pub comment: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct NewComment {
    pub user_id: String,
    pub book_id: String,
    pub comment: String,
    pub rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct SearchByRating {
    pub operator: String,
    pub rating: f64,
}

impl From<NewComment> for Comment {
    fn from(value: NewComment) -> Self {
        Comment {
            user_id: value.user_id,
            book_id: value.book_id,
            comment: value.comment,
            rating: value.rating,
        }
    }
}

#[rocket::post("/api/comment", data = "<comment>")]
pub async fn create_comment(comment: Json<NewComment>, db: &State<Mongo>) -> Result<Json<Comment>, Debug<Box<dyn Error>>> {
    let new_comment = db.create_comment(comment.into_inner()).await?;
    Ok(Json(new_comment))
}

#[rocket::get("/api/comment")]
pub async fn get_comments(db: &State<Mongo>) -> Result<Json<Vec<Comment>>, Debug<Box<dyn Error>>> {
    let comments = db.get_all_comments().await?;
    Ok(Json(comments))
}

#[rocket::get("/api/comment/<book_id>")]
pub async fn get_comments_by_book_id(book_id: &str, db: &State<Mongo>) -> Result<Json<Vec<Comment>>, Debug<Box<dyn Error>>> {
    let comments = db.get_all_comments_with_book_id(book_id).await?;
    Ok(Json(comments))
}

#[rocket::get("/api/comment/user/<user_id>")]
pub async fn get_comments_by_user_id(user_id: &str, db: &State<Mongo>) -> Result<Json<Vec<Comment>>, Debug<Box<dyn Error>>> {
    let comments = db.get_all_comments_with_user_id(user_id).await?;
    Ok(Json(comments))
}

#[rocket::get("/api/comment/rating/<book_id>")]
pub async fn get_rating_by_book_id(book_id: &str, db: &State<Mongo>) -> Result<Json<f64>, Debug<Box<dyn Error>>> {

    let rating = db.calculate_rating_by_book_id(book_id).await?;
    Ok(Json(rating))
}

#[rocket::get("/api/comment/search/rating", data = "<search_by_rating>")]
pub async fn get_all_books_by_search_rating(search_by_rating: Json<SearchByRating>, db: &State<Mongo>) -> Result<Json<Vec<Book>>, Debug<Box<dyn Error>>> {
    let value = &search_by_rating.clone().rating;
    let rating = match search_by_rating.operator.as_str() {
        "=" => OperatorRating::Equal(*value),
        "!=" => OperatorRating::NotEqual(*value),
        ">" => OperatorRating::Greater(*value),
        ">=" => OperatorRating::GreaterOrEqual(*value),
        "<" => OperatorRating::Less(*value),
        "<=" => OperatorRating::LessOrEqual(*value),
        _ => OperatorRating::Equal(*value),
    };
    let comments = db.get_all_books_by_operator_rating(rating).await?;

    Ok(Json(comments))
}
