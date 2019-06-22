extern crate froovie_db;
extern crate serde_json;

use crate::dtos::SearchQuery;
use crate::dtos::movies::SearchResultMovieDto;
use froovie_db::user_selections;
use froovie_db::users;

use nickel::{
    HttpRouter, JsonBody, MiddlewareResult, Nickel, QueryString, Request, Response, Router,
};

use crate::dtos::user_selections::NewUserSelectionDto;
use crate::dtos::user_selections::UserSelectionDto;
use crate::dtos::users::NewUserDto;
use crate::dtos::users::UserDto;
use crate::dtos::FromModel;
use crate::dtos::ToModel;

use crate::nickel::status::StatusCode;
use crate::services::tmdb_fetcher;

pub fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Set appropriate headers
    res.headers_mut()
        .set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    res.headers_mut()
        .set_raw("Access-Control-Allow-Methods", vec![b"*".to_vec()]);
    res.headers_mut().set_raw(
        "Access-Control-Allow-Headers",
        vec![b"Origin, X-Requested-With, Content-Type, Accept".to_vec()],
    );

    // Pass control to the next middleware
    res.next_middleware()
}

pub fn get_router() -> Router {
    let mut router = Nickel::router();

    router.get(
        "/users",
        middleware! { |_request|
            let users = users::all();
            let dtos: Vec<UserDto>= users.into_iter()
                .map(|user| user.id)
                .map(UserDto::from_model)
                .collect();
            serde_json::to_string(&dtos).unwrap()
        },
    );

    router.get("/users/:id", middleware! { |request|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
        let user_dto: UserDto = UserDto::from_model(user_id);
        serde_json::to_string(&user_dto).unwrap();
    });

    router.get("/users/:id/selections", middleware! { |request|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
        info!("Getting user {} selection", user_id); 
        let user_selections = UserSelectionDto::from_model(user_id);
        serde_json::to_string(&user_selections).unwrap()

    });

    router.post(
        "/users/selections",
        middleware! { |request, response|
            info!("inserting user selection");
            let selection_dto = try_with!(response, {
                request.json_as::<NewUserSelectionDto>().map_err(|e| (StatusCode::BadRequest, e))
            });

            let id = user_selections::insert(selection_dto.to_model());
            let user_selections = UserSelectionDto::from_model(selection_dto.user_id);
            serde_json::to_string(&user_selections).unwrap()
        },
    );

    router.post(
        "/users",
        middleware! { |request, response|
            let user_dto = try_with!(response, {
                request.json_as::<NewUserDto>().map_err(|e| (StatusCode::BadRequest, e))
            });

            let db_user = users::insert(&user_dto.to_model());
            let result_dto = UserDto::from_model(db_user.id);
            serde_json::to_string(&result_dto).unwrap()
        },
    );

    router.post(
        "/movies/search",
        middleware! { |request, response|

        let query = try_with!(response, {
            request.json_as::<SearchQuery>().map_err(|e| (StatusCode::BadRequest, e))
        });
            let search_results = tmdb_fetcher::search(query.value);

        if let Ok(result) = search_results {
            let search_result_dtos = SearchResultMovieDto::from_tmdb_search(&result);
            (StatusCode::Ok, serde_json::to_string(&search_result_dtos).unwrap())
        } else {
            (StatusCode::NotFound, "no result found".to_string())
        }
        },
    );

    router
}
