# Darts

## Description

Personal darts scorer tool.

# Database setup

## Start database under Docker

```
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres --name postgres postgres:16.2
```

## Create database

```
sqlx database create --database-url=$DEV_DATABASE_URL
sqlx database create --database-url=$TEST_DATABASE_URL
```

## Drop database

```
sqlx database drop --database-url=$DEV_DATABASE_URL
sqlx database drop --database-url=$TEST_DATABASE_URL
```

## Run migrations

```
sqlx migrate run --database-url=$DEV_DATABASE_URL
sqlx migrate run --database-url=$TEST_DATABASE_URL
```

## Revert migrations

```
sqlx migrate revert --database-url=$DEV_DATABASE_URL
sqlx migrate revert --database-url=$TEST_DATABASE_URL
```

## Run tests

```
DATABASE_URL=$TEST_DATABASE_URL cargo test
```
