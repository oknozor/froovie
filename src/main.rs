#[macro_use]
extern crate nickel;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use nickel::{HttpRouter, Nickel, Request, Response, MiddlewareResult, JsonBody, MediaType};

extern crate froovie_db;

use froovie_db::users::*;
use froovie_db::*;

mod movies;
mod dtos;
mod mappers;

use movies::*;
use dtos::UserDto;
use dtos::NewUserDto;
use nickel::status::StatusCode;
use crate::mappers::FromModel;
use crate::mappers::ToModel;

fn main() {
    let mut server = Nickel::new();
    server.utilize(add_json_header);

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
        serde_json::to_string(&user_dto).unwrap()
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
        let movie = movies::search();
        format!("{}", movie.title)
    });

    server.utilize(router);

    server.listen("127.0.0.1:6767").unwrap();
}

fn add_json_header<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.headers_mut().set_raw("Content-Type", vec![b"Application/json,".to_vec()]);
    res.next_middleware()
}