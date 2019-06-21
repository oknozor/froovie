extern crate reqwest;
extern crate tmdb;

use dotenv::dotenv;

use std::collections::HashMap;
use std::env;
use tmdb::model::*;
use tmdb::themoviedb::*;

//TODO: refactor using config struct
lazy_static! {
    static ref HASMAP: HashMap<u32, String> = {
        dotenv().ok();
        let api_key = env::var("TMDB_API_KEY").expect("TMDB api key must be set");
        let mut m = HashMap::new();
        m.insert(0, api_key);
        m
    };
}

pub fn search(search_query: &str) -> SearchResult {
    let tmdb = TMDb {
        api_key: HASMAP.get(&0).unwrap(),
        language: "fr",
    };
    info!("Fetching tmbd search query :\"{}\"", search_query);

    tmdb.search().title(search_query).execute().unwrap()
}

pub fn by_id(movie_id: u64) -> Movie {
    let tmdb = TMDb {
        api_key: HASMAP.get(&0).unwrap(),
        language: "fr",
    };

    info!("Fetching tmbd movie with id :\"{}\"", movie_id);

    tmdb.fetch().id(movie_id).execute().unwrap()
}

pub fn get_image(movie_id: u64) -> String {
    format!(
        "https://api.themoviedb.org/3/movie/{}/images?api_key={}&language=en-US",
        movie_id,
        HASMAP.get(&0).unwrap()
    )
}
