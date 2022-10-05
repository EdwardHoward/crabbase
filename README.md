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

`diesel setup`

`diesel migration run`

### Install and build admin ui

`cd ui`

`yarn install`

`yarn build`

### Start crabbase from root

`cargo start`

## Development
### Hot reloading

`cargo watch -x "run" -w "./src"`

If running inside a docker container, you might need to poll for watch to work

`cargo watch --poll -x "run" -w "./src"`
