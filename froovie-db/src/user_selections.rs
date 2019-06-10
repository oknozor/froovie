use crate::establish_connection;
use crate::schema::user_selections::dsl::*;
use diesel::prelude::*;
use super::schema::user_selections;
use super::users::User;

#[derive(Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(User)]
pub struct UserSelection {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub chosen: bool,
}

#[derive(Queryable, Associations)]
#[belongs_to(User)]
#[table_name="user_selections"]
pub struct NewUserSelection {
    pub user_id: i32,
    pub movie_id: i32,
}


pub fn by_user_id(in_user_id: i32) -> Vec<UserSelection> {
    let connection = establish_connection();
    let user: User = super::users::find_by_id(in_user_id);
    UserSelection::belonging_to(&user)
        .load::<UserSelection>(&connection)
        .expect("Error loading user_selection")
}

pub fn find_by_id(selection_id: i32) -> UserSelection {
    let connection = establish_connection();
    user_selections
        .find(selection_id)
        .first::<UserSelection>(&connection).expect("Error loading user_selection")
}


pub fn insert(selection: NewUserSelection) -> i32 {
    let connection = establish_connection();

    diesel::insert_into(user_selections::table)
        .values((
            user_selections::user_id.eq(selection.user_id),
            user_selections::movie_id.eq(selection.movie_id)
        ))
        .returning(user_selections::id)
        .get_result::<i32>(&connection)
        .expect("Error saving new users")
}



