# sqlx_demo

[sqlx](https://github.com/launchbadge/sqlx)

## prepare

### postgreSQL

```yaml
version: '3'

services:
  database:
    image: 'postgres'
    ports:
      - '5432:5432'
    restart: always
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=todos
    volumes:
      - database_data:/var/lib/postgresql/data
volumes:
  database_data:
    driver: local
```

```sh
docker compose up # 0.0.0.0:5432
```

### pgAdmin

```sh
docker pull dpage/pgadmin4
docker run -d -p 5433:80 --name pgadmin4 -e PGADMIN_DEFAULT_EMAIL=test@123.com -e PGADMIN_DEFAULT_PASSWORD=123456 dpage/pgadmin4 # 0.0.0.0:5433
```

## installation

```bash
> cargo binstall sqlx-cli
> cargo sqlx -h
```

```sh
cargo sqlx db create # create datebase
cargo sqlx migrate run # create table
```

## usage

Add a todo

```sh
cargo run -- add "todo description"
```

Complete a todo

```sh
cargo run -- done <todo id>
```

List all todos

```sh
cargo run
```
