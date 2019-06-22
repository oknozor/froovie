
use froovie_db::movies::Movie;
use super::movies::MovieDto;
use super::users::UserDto;
use super::FromModel;
use super::ToModel;
use crate::services::tmdb_fetcher;
use froovie_db::movies;
use froovie_db::movies::NewMovie;
use froovie_db::user_selections;
use froovie_db::user_selections::{NewUserSelection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSelectionDto {
    pub user: UserDto,
    pub movies: Vec<MovieDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserSelectionDto {
    pub user_id: i32,
    pub moviedb_id: i32,
}

impl FromModel<Movie> for UserSelectionDto {
    fn from_model(user_id: i32) -> Self {
        let user = UserDto::from_model(user_id);
        let selections = user_selections::by_user_id(user_id);

        let movies = selections.iter()
            .map(|selection| selection.movie_id)
            .map(MovieDto::from_model)
            .collect();

        UserSelectionDto { user, movies }
    }
}

impl<'a> ToModel<'a, NewUserSelection> for NewUserSelectionDto {
    fn to_model(&self) -> NewUserSelection {
        let tmdb_movie = tmdb_fetcher::by_id(self.moviedb_id as u64);
        movies::insert(NewMovie {
            moviedb_id: tmdb_movie.id as i32,
            title: tmdb_movie.title,
            description: tmdb_movie.overview,
            image_url: tmdb_movie.backdrop_path,
        });
        let movie = movies::find_by_tmdb_id(self.moviedb_id);
        NewUserSelection {
            user_id: self.user_id,
            movie_id: movie.id,
        }
    }
}
