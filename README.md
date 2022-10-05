# Crabbase

## Install

### .env

Create a `.env` file in the root of the project

``` env
DATABASE_URL=data.db
HOST=localhost
PORT=8080
```

### Setup [Diesel](http://diesel.rs) (Database ORM)

`cargo install diesel_cli`

`diesel migration run`

### Install and build admin ui

`cd ui`

`yarn install`

`yarn build`

# Run
### Start crabbase from root

`cargo start`

> This might crash at first if you don't have the data.db file generated, but it should generate that file and start working if you run it a second time.


| page | url  |
| ---- | ---- |
| admin | `http://localhost:8080/_`|
| api | `http://localhost:8080/api`|

# Development
### Hot reloading

`cargo watch -x "run" -w "./src"`

If running inside a docker container, you might need to poll for watch to work

`cargo watch --poll -x "run" -w "./src"`
