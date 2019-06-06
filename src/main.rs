#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get(
        "/users",
        middleware! { |request|
            let users = all();
            users.iter()
            .map(|user| user.to_string())
            .collect::<String>()
        },
    );

    router.get("/users/:id", middleware! { |request, response|
        let user_id: i32 = request.param("id").expect("invalid user id").parse::<i32>().unwrap();
        let user = by_id(user_id);
        user.to_string()
    });

    server.utilize(router);

    server.listen("127.0.0.1:6767").unwrap();
}

extern crate diesel;
extern crate froovie_db;

use diesel::prelude::*;
use froovie_db::models::*;
use froovie_db::*;

fn by_id(user_id: i32) -> User {
    use froovie_db::schema::users::dsl::*;

    let connection = establish_connection();
    users
        .find(user_id)
        .get_result::<User>(&connection)
        .expect("Error loading users")
}

fn all() -> Vec<User> {
    use froovie_db::schema::users::dsl::*;

    let connection = establish_connection();
    users
        .load::<User>(&connection)
        .expect("Error loading users")
}
