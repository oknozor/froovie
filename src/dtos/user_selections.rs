
use super::movies::MovieDto;
use super::users::UserDto;
use super::FromModel;
use super::ToModel;
use crate::services::tmdb_fetcher;
use froovie_db::movies;
use froovie_db::movies::NewMovie;
use froovie_db::user_selections::{NewUserSelection, UserSelection};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSelectionDto {
    pub user: UserDto,
    pub movie: MovieDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserSelectionDto {
    pub user_id: i32,
    pub moviedb_id: i32,
}

impl FromModel<UserSelection> for UserSelectionDto {
    fn from_model(selection_model: UserSelection) -> Self {
        let user = UserDto::from_model(froovie_db::users::find_by_id(selection_model.user_id));
        let movies_entity = movies::find_by_id(selection_model.id)
            .unwrap_or_else(|_| panic!("no movie with id {}", selection_model.id));
        let movie = MovieDto::from_model(movies_entity);
        UserSelectionDto { user, movie }
    }
}

impl<'a> ToModel<'a, NewUserSelection> for NewUserSelectionDto {
    fn to_model(&self) -> NewUserSelection {
        let tmdb_movie = tmdb_fetcher::by_id(self.moviedb_id as u64);
        movies::insert(NewMovie {
            moviedb_id: tmdb_movie.id as i32,
            title: tmdb_movie.title,
            description: tmdb_movie.overview,
        });
        let movie = movies::find_by_tmdb_id(self.moviedb_id);
        NewUserSelection {
            user_id: self.user_id,
            movie_id: movie.id,
        }
    }
}