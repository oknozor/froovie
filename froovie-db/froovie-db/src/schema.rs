table! {
    movie_night (id) {
        id -> Int4,
        movie_time -> Nullable<Date>,
    }
}

table! {
    movies (id) {
        id -> Int4,
        moviedb_id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        nick -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

table! {
    user_selections (id) {
        id -> Int4,
        user_id -> Int4,
        movie_id -> Int4,
        chosen -> Bool,
    }
}

joinable!(user_selections -> movies (movie_id));
joinable!(user_selections -> users (user_id));

allow_tables_to_appear_in_same_query!(
    movie_night,
    movies,
    users,
    user_selections,
);
