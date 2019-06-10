extern crate froovie_db;
use serde::{Serialize, Deserialize};
use froovie_db::users::{User, NewUser};
use crate::mappers::FromModel;
use crate::mappers::ToModel;
use std::error::Error;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: Option<i32>,
    pub nick: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserDto {
    pub nick: String,
    pub email: String,
    pub password: String,
}

impl FromModel<User> for UserDto {
    fn from_model(user: User) -> Self {
        UserDto {
            id: Some(user.id),
            nick: user.nick,
            email: user.email,
        }
    }
}

impl <'a> ToModel<'a, NewUser<'a>> for NewUserDto {
    fn to_model(&'a self) -> NewUser<'a> {
        let password_hash = hash(self.password.as_str(), DEFAULT_COST)
            .expect("crypto erro");
        NewUser {
            nick: self.nick.as_str(),
            email: selfword .email.as_str(),
            password_hash
        }
    }
}