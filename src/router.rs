extern crate froovie_db;
extern crate serde_json;

use froovie_db::*;
use froovie_db::users::*;
use nickel::{HttpRouter, Nickel, Request, Response, MiddlewareResult, JsonBody, MediaType, Router};

use crate::dtos::NewUserDto;
use crate::dtos::UserDto;
use crate::mappers::FromModel;
use crate::mappers::ToModel;
use crate::movies_client::*;
use crate::nickel::status::StatusCode;


pub fn get_router() -> Router {
    let mut router = Nickel::router();

    router.get(
        "/users",
        middleware! { |request|
            let users = all();
            let dtos :Vec<UserDto>= users.iter()
                .map(|user| UserDto::from_model(user.clone()))
                .collect();
            serde_json::to_string(&dtos).unwrap()
        },
    );

    router.get("/users/:id", middleware! { |request|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
        let user_dto: UserDto = UserDto::from_model(by_id(user_id));
        let user = serde_json::to_string(&user_dto).unwrap();
        user
    });

    router.post("/users", middleware! { |request, response|
        let user_dto = try_with!(response, {
            request.json_as::<NewUserDto>().map_err(|e| (StatusCode::BadRequest, e))
        });

        let db_user = new(&user_dto.to_model());
        let result_dto = UserDto::from_model(db_user);
        serde_json::to_string(&result_dto).unwrap()
    });


    router.post("/movies/:search", middleware! { |request|
        let search_query: &str = request.param("search").expect("invalid search query");
        let movie = search();
        format!("{}", movie.title)
    });

    router
}