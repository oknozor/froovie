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
cd froovie-db && 
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


