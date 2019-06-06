## Setup 

create the postgresql database
```
CREATE USER froovie WITH PASSWORD 'froovie';
CREATE DATABASE froovie WITH OWNER = froovie ENCODING = 'UTF8' TABLESPACE = pg_default CONNECTION LIMIT = -1;
```

Install the diesel-cli
```
cargo install diesel_cli --no-default-features --features postgres
```

Add the database endpoint to the project environment
```
echo DATABASE_URL=postgres://froovie:password@localhost/froovie > .env
```

Generate diesel migration
```
diesel setup
```