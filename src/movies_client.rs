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

//TODO
pub fn search() -> Movie {

    let tmdb = TMDb { api_key: HASMAP.get(&0).unwrap(), language: "fr" };

    let search_result = tmdb.search()
        .title("Interstellar")
        .year(2014)
        .execute()
        .unwrap();

    let id = search_result.results[0].id;

    tmdb.fetch()
        .id(id)
        .execute()
        .unwrap()
}
