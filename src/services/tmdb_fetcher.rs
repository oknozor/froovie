extern crate tmdb;

use tmdb::model::*;
use tmdb::themoviedb::*;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;


//TODO: refactor using config struct
lazy_static! {
    static ref HASMAP: HashMap<u32, String> = {
        dotenv().ok();
        let api_key = env::var("TMDB_API_KEY")
            .expect("TMDB api key must be set");
        let mut m = HashMap::new();
        m.insert(0, api_key);
        m
    };
}

pub fn search(search_query: &str) -> SearchResult {
    let tmdb = TMDb { api_key: HASMAP.get(&0).unwrap(), language: "fr" };
    tmdb.search()
        .title(search_query)
        .execute()
        .unwrap()
}

pub fn by_id(movie_id: u64) -> Movie {
    let tmdb = TMDb { api_key: HASMAP.get(&0).unwrap(), language: "fr" };

    tmdb.fetch()
        .id(movie_id)
        .execute()
        .unwrap()
}