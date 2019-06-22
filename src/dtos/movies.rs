
use tmdb::model::SearchResult;
use froovie_db::movies;
use froovie_db::movies::{Movie, NewMovie};
use serde::{Deserialize, Serialize};

use super::FromModel;
use super::ToModel;
use crate::services::tmdb_fetcher;

const MOVIE_DB_IMG_URL: &str = "https://image.tmdb.org/t/p/original";

#[derive(Serialize, Deserialize, Debug)]
pub struct MovieDto {
    pub id: i32,
    pub moviedb_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResultMovieDto {
    pub moviedb_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct NewMovieDto {
    pub moviedb_id: i32,
}

impl SearchResultMovieDto {
    pub fn from_tmdb_search(search_result: &SearchResult) -> Vec<Self> {
        search_result.results
        .iter()
        .map(|result| SearchResultMovieDto {
            moviedb_id: result.id, 
            description: result.overview.to_owned(),
            title: result.title.clone(), 
            image_url: if let Some(img) = &result.poster_path {
                Some(format!("{}{}", MOVIE_DB_IMG_URL, img))
            } else {
                None
            }
        }).collect()
    } 
}

impl FromModel<Movie> for MovieDto {
    fn from_model(movie_id: i32) -> Self {
        let movie = movies::find_by_id(movie_id)
            .unwrap_or_else(|_| panic!("Unable to find movie with id {}", movie_id));
        MovieDto {
            id: movie.id,
            moviedb_id: movie.moviedb_id,
            title: movie.title.clone(),
            description: movie.description,
            image_url: movie.image_url,
        }
    }
}

impl<'a> ToModel<'a, NewMovie> for NewMovieDto {
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
