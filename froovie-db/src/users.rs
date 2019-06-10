use crate::establish_connection;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use super::schema::users;
use crate::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32, 
    pub nick: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub nick: &'a str,
    pub email: &'a str,
    pub password_hash: String,
}


impl ToString for User {
    fn to_string(&self) -> String {
        format!("id : {} , nick : {} ", self.id, self.nick) 
    }
}

//TODO: refactor this to Option<User>
pub fn find_by_id(user_id: i32) -> User {

    let connection = establish_connection();
    users
        .find(user_id)
        .get_result::<User>(&connection)
        .expect("Error loading users")
}

pub fn all() -> Vec<User> {

    let connection = establish_connection();
    users
        .load::<User>(&connection)
        .expect("Error loading users")
}

pub fn insert<'a>(new_user: &NewUser) -> User {
    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection)
        .expect("Error saving new users")
}
