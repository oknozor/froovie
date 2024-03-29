
use super::FromModel;
use super::ToModel;
use bcrypt::{hash, DEFAULT_COST};
use froovie_db::users;
use froovie_db::users::{NewUser, User};
use serde::{Deserialize, Serialize};

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
    fn from_model(user_id: i32) -> Self {
        let user = users::find_by_id(user_id); 
        UserDto {
            id: Some(user.id),
            nick: user.nick,
            email: user.email,
        }
    }
}

impl<'a> ToModel<'a, NewUser<'a>> for NewUserDto {
    fn to_model(&'a self) -> NewUser<'a> {
        let password_hash = hash(self.password.as_str(), DEFAULT_COST).expect("crypto error");
        NewUser {
            nick: self.nick.as_str(),
            email: self.email.as_str(),
            password_hash,
        }
    }
}

