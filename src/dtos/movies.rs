use serde::{Serialize, Deserialize};
use froovie_db::movies::{Movie, NewMovie};

use crate::services::tmdb_fetcher;
use super::FromModel;
use super::ToModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct MovieDto {
    pub id: i32,
    pub moviedb_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewMovieDto {
    pub moviedb_id: i32,
}

impl FromModel<Movie> for MovieDto {
    fn from_model(movie: Movie) -> Self {
        MovieDto {
            id: movie.id,
            moviedb_id: movie.moviedb_id,
            title: movie.title.clone(),
            description: movie.description,
            image_url: movie.image_url
        }
    }
}

impl <'a> ToModel<'a, NewMovie> for NewMovieDto {
    fn to_model(&'a self) -> NewMovie {
        let tmdb_movie: tmdb::model::Movie = tmdb_fetcher::by_id(self.moviedb_id as u64);

        NewMovie {
            moviedb_id: tmdb_movie.id as i32,
            title: tmdb_movie.title.clone(),
            description: tmdb_movie.overview.clone(),
            image_url: tmdb_movie.backdrop_path,
        }

    }
}
