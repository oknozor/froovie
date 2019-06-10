use serde::{Serialize, Deserialize};
use froovie_db::users::{User, NewUser};
use bcrypt::{hash, DEFAULT_COST};
use super::FromModel;
use super::ToModel;

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
            .expect("crypto error");
        NewUser {
            nick: self.nick.as_str(),
            email: self.email.as_str(),
            password_hash
        }
    }
}
