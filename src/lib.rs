pub mod genre;
pub mod book;
pub mod user;
pub mod comment;
pub mod mongo;

pub enum Value {
    Int(i32),
    Bool(bool),
    Text(String),
}

pub enum OperatorRating {
    Equal(f64),
    NotEqual(f64),
    Greater(f64),
    GreaterOrEqual(f64),
    Less(f64),
    LessOrEqual(f64),
}