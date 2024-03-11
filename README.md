## Create database

```
sqlx database create --database-url=postgres://postgres:$PG_PASS@localhost/darts
sqlx database create --database-url=postgres://postgres:$PG_PASS@localhost/darts_test
```

## Drop database

```
sqlx database create --database-url=postgres://postgres:$PG_PASS@localhost/darts
sqlx database create --database-url=postgres://postgres:$PG_PASS@localhost/darts_test
```

## Run migrations

```
sqlx migrate run --database-url=postgres://postgres:$PG_PASS@localhost/darts
sqlx migrate run --database-url=postgres://postgres:$PG_PASS@localhost/darts_test
```

## Revert migrations

```
sqlx migrate revert --database-url=postgres://postgres:$PG_PASS@localhost/darts
```

## Run tests

```
DATABASE_URL=postgres://postgres:$PG_PASS@localhost/darts_test cargo test --package dataspine --test create_game_test -- it_creats_game --exact --nocapture
```

```
DATABASE_URL=postgres://postgres:$PG_PASS@localhost/darts_test cargo test --package dataspine
```
