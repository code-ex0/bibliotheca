use bibliotheca::mongo::BuildMongo;
use bibliotheca::book::{create_book, get_books, get_book, search_book, update_book, delete_book, borrow_book, return_book};
use bibliotheca::user::{create_user, get_users, delete_user, update_user, search_user};
use bibliotheca::genre::{create_genre, get_genres, get_books_by_genre};
use bibliotheca::comment::{create_comment, get_comments, get_comments_by_book_id, get_comments_by_user_id, get_rating_by_book_id, get_all_books_by_search_rating};

// no main function
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let mongo = BuildMongo::new().await.unwrap().build();

    rocket::build()
        .mount("/", routes![create_book, get_books, get_book, search_book, delete_book, update_book, borrow_book, return_book])
        .mount("/", routes![create_user, get_users, delete_user, update_user, search_user])
        .mount("/", routes![create_genre, get_genres, get_books_by_genre])
        .mount("/", routes![create_comment, get_comments, get_comments_by_book_id, get_comments_by_user_id, get_rating_by_book_id, get_all_books_by_search_rating])
        .manage(mongo)
}
