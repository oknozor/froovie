pub mod movies;
pub mod user_selections;
pub mod users;
use serde::{Deserialize, Serialize};


pub trait FromModel<Model> {
    fn from_model(id: i32) -> Self;
}

pub trait ToModel<'a, Model> {
    fn to_model(&'a self) -> Model;
}


#[derive(Deserialize, Debug)]
pub struct SearchQuery<'a> { 
    pub value: &'a str,
}