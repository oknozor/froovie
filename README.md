# Froovie 

## Development setup

### Database

1. create the postgresql database : 
```sql 
CREATE USER froovie WITH PASSWORD 'froovie';
CREATE DATABASE froovie WITH OWNER = froovie ENCODING = 'UTF8' TABLESPACE = pg_default CONNECTION LIMIT = -1;
```

2. Install the diesel-cli : 
```sh
cargo install diesel_cli --no-default-features --features postgres
```

3. Add the database endpoint to the project environment : 
```sh
echo DATABASE_URL=postgres://froovie:password@localhost/froovie > .env
```

3. Run the diesel migrations :
```sh
diesel migration run 
```

if you have previously ran the migration use `diesel migration redo` instead.

### Movie database client

add the tmbd api key to the dotenv : 
```sh
echo "TMDB_API_KEY={my-api-key}" >> .env 
```

### Run 

```
cargo run
```

## TODO 

- rename mapper/dtos to something more appropriate
- add [jwt auth](https://www.crates.io/crates/nickel-jwt-session)
- find an elegant way to unit test controllers (mockito rust ?)
- add tooling (CI, rustfmt, clippy...etc, gerkhin)
- maybe split model into a dedicated library
- do some research on testing with diesel (test migration, h2 integration)
- frontend : we can build a nice rusty frontend with [Yew](https://github.com/DenisKolodin/yew) 

