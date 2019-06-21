extern crate froovie_db;
extern crate serde_json;

use froovie_db::user_selections;
use froovie_db::users;

use nickel::{HttpRouter, JsonBody, Nickel, Router};

use crate::dtos::user_selections::NewUserSelectionDto;
use crate::dtos::user_selections::UserSelectionDto;
use crate::dtos::users::NewUserDto;
use crate::dtos::users::UserDto;
use crate::dtos::FromModel;
use crate::dtos::ToModel;

use crate::nickel::status::StatusCode;
use crate::services::tmdb_fetcher;

pub fn get_router() -> Router {
    let mut router = Nickel::router();

    router.get(
        "/users",
        middleware! { |_request|
            let users = users::all();
            let dtos: Vec<UserDto>= users.into_iter()
                .map(UserDto::from_model)
                .collect();
            serde_json::to_string(&dtos).unwrap()
        },
    );

    router.get("/users/:id", middleware! { |request|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
        let user_dto: UserDto = UserDto::from_model(users::find_by_id(user_id));
        serde_json::to_string(&user_dto).unwrap();
    });

    router.get("/users/:id/selections", middleware! { |request|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
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
            let result_dto = UserDto::from_model(db_user);
            serde_json::to_string(&result_dto).unwrap()
        },
    );

    router.post(
        "/movies/:search",
        middleware! { |request|
            let search_query: &str = request.param("search").expect("invalid search query");
            let search_results = tmdb_fetcher::search(search_query);
            serde_json::to_string(&search_results).unwrap()
        },
    );

    router
}
