# encapsulate-api
Backend for encapsulate mobile app

To run this, you need rust and cargo

Install both with:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
https://www.rust-lang.org/tools/install


To run the app:

```sh
cargo run
```


DB and migrations:

Need a posgtres db

Run with docker:

```sh
docker run --name postgresql-container -p 5555:5432 -e POSTGRES_PASSWORD=pass -d postgres
```
Then create a .env file in root with the following env vars:

```sh
DATABASE_URL=postgres://postgres:pass@localhost:5555/encapsulate
```

For migrations, need diesel-cli:
https://diesel.rs/guides/getting-started
```sh
cargo install diesel_cli --no-default-features --features postgres

diesel setup
diesel migration run
````

https://github.com/actix/examples


### Latest Changes

* Add simple auth and todos. PR [#3](https://github.com/haffi96/encapsulate-api/pull/3) by [@haffi96](https://github.com/haffi96).
* add user table. PR [#2](https://github.com/haffi96/encapsulate-api/pull/2) by [@haffi96](https://github.com/haffi96).
