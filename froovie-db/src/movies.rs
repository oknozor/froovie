use super::schema::movies;
use crate::establish_connection;
use crate::schema::movies::dsl::*;
use diesel::prelude::*;
use crate::*;
use diesel::result::Error;


#[derive(Queryable, PartialEq, Debug)]
pub struct Movie {
    pub id: i32,
    pub moviedb_id: i32,
    pub title: String,
    pub description: Option<String>
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie{
    pub moviedb_id: i32,
    pub title: String,
    pub description: Option<String>
}


pub fn insert(movie: NewMovie) -> Movie {
    let connection = establish_connection();
    diesel::insert_into(movies::table)
        .values(movie)
        .get_result(&connection)
        .expect("Error saving new movie")
}

pub fn find_by_id(movie_id: i32) -> Result<Movie, Error> {
    let connection = establish_connection();
    movies
        .find(movie_id)
        .get_result::<Movie>(&connection)
}

pub fn find_by_tmdb_id(tmdb_id: i32) -> Movie {
    println!("mdb id {}", tmdb_id);
    let connection = establish_connection();
    movies
        .filter(moviedb_id.eq(tmdb_id))
        .first::<Movie>(&connection)
        .expect("Error loading movie")
}