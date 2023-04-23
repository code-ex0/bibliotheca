use std::collections::HashMap;
use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use bson::{doc, Document};
use rocket::futures::StreamExt;
use crate::book::{Book, NewBook};
use crate::comment::{Comment, NewComment};
use crate::genre::Genre;
use crate::user::{NewUser, User};
use crate::{OperatorRating, Value};

pub struct Config {
    pub url: String,
    pub db_name: String,
    pub collection_name: String,
}

pub struct BuildConfig {
    pub url: String,
    pub db_name: String,
    pub collection_name: String,
}

pub struct Mongo {
    pub config: Config,
    pub client: Client,
}


pub struct BuildMongo {
    pub config: BuildConfig,
    pub client: Client,
}

impl BuildConfig {

    ///
    /// # build
    /// this function build a config struct and return a config struct
    /// # Return
    /// * `Config` - a config struct
    ///
    fn build(self) -> Config {
        Config {
            url: self.url,
            db_name: self.db_name,
            collection_name: self.collection_name,
        }
    }

    ///
    /// # new
    /// this function create a new build config struct and return a build config struct or an error
    /// # Return
    /// * `Result<Self, Box<dyn Error>>` - a build config struct or an error
    ///
    fn new() -> Result<Self, Box<dyn Error>>
    {
        let url = env::var("URL_MONGO")?;
        let db_name = env::var("DB_NAME")?;
        let collection_name = env::var("COLLECTION_NAME")?;

        Ok(BuildConfig {
            url,
            db_name,
            collection_name,
        })
    }
}

impl BuildMongo {

    ///
    /// # build
    /// this function build a mongo struct and return a mongo struct
    /// # Arguments
    /// * `self` - the build mongo struct
    /// # Return
    /// * `Mongo` - a mongo struct
    ///
    pub fn build(self) -> Mongo {
        Mongo {
            config: self.config.build(),
            client: self.client,
        }
    }

    ///
    /// # new
    /// this function create a new mongo struct and return a mongo struct or an error
    /// # Return
    /// * `Result<BuildMongo, Box<dyn Error>>` - a mongo struct or an error
    ///
    pub async fn new() -> Result<BuildMongo, Box<dyn Error>>
    {
        let config = BuildConfig::new()?;
        let options = ClientOptions::parse_with_resolver_config(&config.url, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;

        Ok(BuildMongo { config, client })
    }
}

impl Mongo {

    // book

    ///
    /// # get all books from database
    /// this function get all books from mongo database and return a vector of books or an error
    ///
    /// # Arguments
    /// * `self` - the mongo struct
    ///
    /// # Return
    /// * `Result<Vec<Book>, Box<dyn Error>>` - a vector of books or an error
    ///
    ///
    pub async fn get_all_books(&self) -> Result<Vec<Book>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let mut cursor = collection.find(None, None).await?;
        let mut books = Vec::new();
        while let Some(result) = cursor.next().await {
            let book = bson::from_bson(bson::Bson::Document(result?))?;
            books.push(book);
        }
        Ok(books)
    }

    ///
    /// # get a book from database
    /// this function get a book with id from mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the book
    /// # Return
    /// * `Result<Book, Box<dyn Error>>` - a book or an error
    ///
    pub async fn get_book_by_id(&self, id: &str) -> Result<Book, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let cursor = collection.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let book = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        Ok(book)
    }


    ///
    /// # create a book in database
    /// this function create a book in mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `book` - the book to create
    /// # Return
    /// * `Result<Book, Box<dyn Error>>` - a book or an error
    ///
    pub async fn create_book(&self, book: NewBook) -> Result<Book, Box<dyn Error>> {
        let book = Book::from(book);
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let doc = bson::to_bson(&book)?;
        let doc = doc.as_document().unwrap();
        collection.insert_one(doc.clone(), None).await?;
        Ok(book)
    }

    ///
    /// # update a book in database
    /// this function update a book with id in mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the book
    /// * `book` - the book to update (HashMap<&str, String>)
    /// # Return
    /// * `Result<Book, Box<dyn Error>>` - a book or an error
    ///
    pub async fn update_book(&self, id: &str, book: HashMap<&str, Value>) -> Result<Book, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let mut query = doc! {};
        for (key, value) in book {
            match value {
                Value::Bool(b) => query.insert(key, b),
                Value::Int(i) => query.insert(key, i),
                Value::Text(t) => query.insert(key, t),
            };
        }
        collection.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, doc! {"$set": query}, None).await?;
        let cursor = collection.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let book = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        Ok(book)
    }

    ///
    /// # delete a book from database
    /// this function delete a book with id from mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the book
    /// # Return
    /// * `Result<Book, Box<dyn Error>>` - a book or an error
    pub async fn delete_book(&self, id: &str) -> Result<Book, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let cursor = collection.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let book = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        collection.delete_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        Ok(book)
    }

    ///
    /// # search a book from database
    /// this function search a book with title, author or year of publication from mongo database and return a vector of books or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `search` - the search query (HashMap<&str, String>)
    /// # Return
    /// * `Result<Vec<Book>, Box<dyn Error>>` - a vector of books or an error
    ///
    pub async fn search_book(&self, search: HashMap<&str, String>) -> Result<Vec<Book>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let mut query = doc! {};
        for (key, value) in search {
            query.insert(key, value);
        }
        let mut cursor = collection.find(query, None).await?;
        let mut books = Vec::new();
        while let Some(result) = cursor.next().await {
            let book = bson::from_bson(bson::Bson::Document(result?))?;
            books.push(book);
        }
        Ok(books)
    }

    ///
    /// # borrow a book from database
    /// this function borrow a book with id from mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the book
    /// * `user_id` - the id of the user
    /// # Return
    /// * `Result<(User, Book), Box<dyn Error>>` - a tuple of user and book or an error
    ///
    pub async fn borrow_book(&self, id: &str, user_id: &str) -> Result<(User, Book), Box<dyn Error>> {
        let collection_book: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let collection_user: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let cursor = collection_book.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let mut book: Book = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        let cursor = collection_user.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(user_id).unwrap()}, None).await?;
        let mut user: User = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;

        if !book.availability {
            return Err("Book not available".into());
        }
        book.availability = false;
        user.borrowed_books.push(id.to_string());

        let doc = bson::to_bson(&book)?;
        let doc = doc.as_document().unwrap();
        collection_book.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, doc! {"$set": doc}, None).await?;


        let doc = bson::to_bson(&user)?;
        let doc = doc.as_document().unwrap();
        collection_user.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(user_id).unwrap()}, doc! {"$set": doc}, None).await?;

        Ok((user, book))
    }

    ///
    /// # return a book from database
    /// this function return a book with id from mongo database and return a book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the book
    /// * `user_id` - the id of the user
    /// # Return
    /// * `Result<(User, Book), Box<dyn Error>>` - a tuple of user and book or an error
    ///
    pub async fn return_book(&self, id: &str, user_id: &str) -> Result<(User, Book), Box<dyn Error>> {
        let collection_book: Collection<Document> = self.client.database(&self.config.db_name).collection("books");
        let collection_user: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let cursor = collection_book.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let mut book: Book = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        let cursor = collection_user.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(user_id).unwrap()}, None).await?;
        let mut user: User = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;

        if book.availability {
            return Err("Book not borrowed".into());
        }
        book.availability = true;
        user.borrowed_books.retain(|x| x != id);

        let doc = bson::to_bson(&book)?;
        let doc = doc.as_document().unwrap();
        collection_book.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, doc! {"$set": doc}, None).await?;

        let doc = bson::to_bson(&user)?;
        let doc = doc.as_document().unwrap();
        collection_user.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(user_id).unwrap()}, doc! {"$set": doc}, None).await?;

        Ok((user, book))
    }
    // end book

    // user

    ///
    /// # create a user in database
    /// this function create a user in mongo database and return a user or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `new_user` - the new user
    /// # Return
    /// * `Result<User, Box<dyn Error>>` - a user or an error
    ///
    pub async fn create_user(&self, new_user: NewUser) -> Result<User, Box<dyn Error>> {
        let user = User::from(new_user);
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("users");

        let date = chrono::NaiveDate::parse_from_str(&user.birth_date, "%Y-%m-%d");
        if date.is_err() {
            return Err("Invalid date format".into());
        }

        let doc = bson::to_bson(&user)?;
        let doc = doc.as_document().unwrap();

        if let Some(_) = collection.find_one(doc! {"email": &user.email}, None).await? {
            return Err("User already exist".into());
        }

        collection.insert_one(doc.clone(), None).await?;
        Ok(user)
    }

    ///
    /// # get all user from database
    /// this function return all user from mongo database and return a vector of user or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// # Return
    /// * `Result<Vec<User>, Box<dyn Error>>` - a vector of user or an error
    ///
    pub async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let mut cursor = collection.find(None, None).await?;
        let mut users = Vec::new();
        while let Some(result) = cursor.next().await {
            let user = bson::from_bson(bson::Bson::Document(result?))?;
            users.push(user);
        }
        Ok(users)
    }

    ///
    /// # search user from database
    /// this function search user from mongo database and return a vector of user or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `search` - the search query (HashMap<&str, String>)
    /// # Return
    /// * `Result<Vec<User>, Box<dyn Error>>` - a vector of user or an error
    ///
    pub async fn search_user(&self, search: HashMap<&str, String>) -> Result<Vec<User>, Box<dyn Error>> {
        let mut query = doc! {};
        for (key, value) in search {
            query.insert(key, value);
        }
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let mut cursor = collection.find(query, None).await?;
        let mut users = Vec::new();
        while let Some(result) = cursor.next().await {
            let user = bson::from_bson(bson::Bson::Document(result?))?;
            users.push(user);
        }
        Ok(users)
    }

    ///
    /// # update user from database
    /// this function update user with id from mongo database and return a user or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `id` - the id of the user
    /// * `user` - the user to update (HashMap<&str, String>)
    /// # Return
    /// * `Result<User, Box<dyn Error>>` - a user or an error
    ///
    pub async fn update_user(&self, id: &str, user: HashMap<&str, String>) -> Result<User, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let mut query = doc! {};
        for (key, value) in user {
            query.insert(key, value);
        }
        collection.update_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, doc! {"$set": query}, None).await?;
        let cursor = collection.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let user = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: &str) -> Result<User, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("users");
        let cursor = collection.find_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        let user = bson::from_bson(bson::Bson::Document(cursor.unwrap()))?;
        collection.delete_one(doc! {"_id": bson::oid::ObjectId::parse_str(id).unwrap()}, None).await?;
        Ok(user)
    }
    // end user

    // comment




    ///
    /// # create a comment in database
    /// this function create a comment in mongo database and return a comment or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `comment` - the new comment
    /// # Return
    /// * `Result<Comment, Box<dyn Error>>` - a comment or an error
    pub async fn create_comment(&self, comment: NewComment) -> Result<Comment, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("comments");
        let comment = Comment::from(comment);
        let doc = bson::to_bson(&comment)?;
        let doc = doc.as_document().unwrap();
        collection.insert_one(doc.clone(), None).await?;
        Ok(comment)
    }

    ///
    /// # get all comment from database
    /// this function return all comment from mongo database and return a vector of comment or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// # Return
    /// * `Result<Vec<Comment>, Box<dyn Error>>` - a vector of comment or an error
    ///
    pub async fn get_all_comments(&self) -> Result<Vec<Comment>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("comments");
        let mut cursor = collection.find(None, None).await?;
        let mut comments = Vec::new();
        while let Some(result) = cursor.next().await {
            let comment = bson::from_bson(bson::Bson::Document(result?))?;
            comments.push(comment);
        }
        Ok(comments)
    }

    ///
    /// # get all comment with book id from database
    /// this function return all comment with book id from mongo database and return a vector of comment or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `book_id` - the id of the book
    /// # Return
    /// * `Result<Vec<Comment>, Box<dyn Error>>` - a vector of comment or an error
    ///
    pub async fn get_all_comments_with_book_id(&self, book_id: &str) -> Result<Vec<Comment>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("comments");
        let mut cursor = collection.find(doc! {"book_id": book_id}, None).await?;
        let mut comments = Vec::new();
        while let Some(result) = cursor.next().await {
            let comment = bson::from_bson(bson::Bson::Document(result?))?;
            comments.push(comment);
        }
        Ok(comments)
    }

    ///
    /// # get all comment with user id from database
    /// this function return all comment with user id from mongo database and return a vector of comment or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `user_id` - the id of the user
    /// # Return
    /// * `Result<Vec<Comment>, Box<dyn Error>>` - a vector of comment or an error
    ///
    pub async fn get_all_comments_with_user_id(&self, user_id: &str) -> Result<Vec<Comment>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("comments");
        let mut cursor = collection.find(doc! {"user_id": user_id}, None).await?;
        let mut comments = Vec::new();
        while let Some(result) = cursor.next().await {
            let comment = bson::from_bson(bson::Bson::Document(result?))?;
            comments.push(comment);
        }
        Ok(comments)
    }

    ///
    /// # get rating by book id from database
    /// this function return rating by book id from mongo database and return a f64 or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `book_id` - the id of the book
    /// # Return
    /// * `Result<f64, Box<dyn Error>>` - a f64 or an error
    ///
    pub async fn calculate_rating_by_book_id(&self, book_id: &str) -> Result<f64, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("comments");
        let mut cursor = collection.find(doc! {"book_id": book_id}, None).await?;
        let mut comments: Vec<Comment> = Vec::new();
        while let Some(result) = cursor.next().await {
            let comment = bson::from_bson(bson::Bson::Document(result?))?;
            comments.push(comment);
        }
        let mut sum = 0.0;
        for comment in &comments {
            sum += comment.clone().rating as f64;
        }
        Ok(sum / comments.len() as f64)
    }

    ///
    /// # get all books by operator rating from database
    /// this function return all books by operator rating from mongo database and return a vector of book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `operator_rating` - the operator rating
    /// # Return
    /// * `Result<Vec<Book>, Box<dyn Error>>` - a vector of book or an error
    ///
    pub async fn get_all_books_by_operator_rating(&self, operator_rating: OperatorRating) -> Result<Vec<Book>, Box<dyn Error>> {

        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("books");

        let operator = match operator_rating {
            OperatorRating::Equal(value) => doc! { "$eq": value },
            OperatorRating::NotEqual(value) => doc! { "$ne": value },
            OperatorRating::Greater(value) => doc! { "$gt": value },
            OperatorRating::GreaterOrEqual(value) => doc! { "$gte": value },
            OperatorRating::Less(value) => doc! { "$lt": value },
            OperatorRating::LessOrEqual(value) => doc! { "$lte": value },
        };

        let pipeline = vec![
            doc! {
                "$addFields": {
                    "book_id_str": { "$toString": "$_id" }
                }
            },
            doc! {
                "$lookup": {
                    "from": "comments",
                    "localField": "book_id_str",
                    "foreignField": "book_id",
                    "as": "comments"
                }
            },
            doc! {
                "$unwind": {
                    "path": "$comments",
                    "preserveNullAndEmptyArrays": true
                }
            },
            doc! {
                "$group": {
                    "_id": "$_id",
                    "title": { "$first": "$title" },
                    "author": { "$first": "$author" },
                    "year": { "$first": "$year" },
                    "resume": { "$first": "$resume" },
                    "availability": { "$first": "$availability" },
                    "average_rating": { "$avg": "$comments.rating" },
                    "gender_id": { "$first": "$gender_id" }
                }
            },
            doc! {
                "$match": {
                    "average_rating": operator
                }
            },
            doc! {
                "$project": {
                    "_id": 1,
                    "title": 1,
                    "author": 1,
                    "year": 1,
                    "resume": 1,
                    "availability": 1,
                    "average_rating": 1,
                    "gender_id": 1
                }
            },
        ];


        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut books = Vec::new();
        while let Some(result) = cursor.next().await {
            let book = bson::from_bson(bson::Bson::Document(result?))?;
            books.push(book);
        }
        Ok(books)
    }

    // end comment

    // genre

    ///
    /// # create genre in database
    /// this function create genre in mongo database and return a genre or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `genre` - the genre to create
    /// # Return
    /// * `Result<Genre, Box<dyn Error>>` - a genre or an error
    ///
    pub async fn create_genre(&self, genre: Genre) -> Result<Genre, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("genres");

        let doc = bson::to_bson(&genre)?;
        let doc = doc.as_document().unwrap();

        if let Some(_) = collection.find_one(doc! {"name": &genre.name}, None).await? {
            return Err("Genre already exist".into());
        }

        collection.insert_one(doc.clone(), None).await?;
        Ok(genre)
    }

    ///
    /// # get all genres from database
    /// this function return all genres from mongo database and return a vector of genre or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// # Return
    /// * `Result<Vec<Genre>, Box<dyn Error>>` - a vector of genre or an error
    ///
    pub async fn get_all_genres(&self) -> Result<Vec<Genre>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("genres");
        let mut cursor = collection.find(None, None).await?;
        let mut genres = Vec::new();
        while let Some(result) = cursor.next().await {
            let genre = bson::from_bson(bson::Bson::Document(result?))?;
            genres.push(genre);
        }
        Ok(genres)
    }

    ///
    /// # get all books by genre from database
    /// this function return all books by genre from mongo database and return a vector of book or an error
    /// # Arguments
    /// * `self` - the mongo struct
    /// * `genre_name` - the genre name
    /// # Return
    /// * `Result<Vec<Book>, Box<dyn Error>>` - a vector of book or an error
    ///
    pub async fn get_books_by_genre(&self, genre_name: &str) -> Result<Vec<Book>, Box<dyn Error>> {
        let collection: Collection<Document> = self.client.database(&self.config.db_name).collection("genres");
        let pipeline = vec![
            doc! {
                "$match": {
                    "name": genre_name
                }
            },
            doc! {
                "$lookup": {
                    "from": "books",
                    "let": { "genre_name": "$name", "genre_id": { "$toString": "$_id" } },
                    "pipeline": [
                        {
                            "$match": {
                                "$expr": {
                                    "$eq": ["$gender_id", "$$genre_id"]
                                }
                            }
                        },
                        {
                            "$project": {
                                "_id": 1,
                                "title": 1,
                                "author": 1,
                                "year": 1,
                                "resume": 1,
                                "availability": 1,
                                "gender_id": "$$genre_name"
                            }
                        }
                    ],
                    "as": "books"
                }
            },
            doc! {
                "$unwind": "$books"
            },
            doc! {
                "$replaceRoot": {
                    "newRoot": "$books"
                }
            },
        ];

        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut books = Vec::new();
        while let Some(result) = cursor.next().await {
            let book = bson::from_bson(bson::Bson::Document(result?))?;
            books.push(book);
        }
        Ok(books)
    }
    // end genre
}