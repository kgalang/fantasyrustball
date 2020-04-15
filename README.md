# Fantasy Rustball

Fantasy football but for your local running group!

An Actix 2.0 REST server using the Rust language.

## What is this app for?

**For Fun:** I want to create an API that will help gamify running data in a fantasy football format. I want groups of people to be able to create their own league, draft their peers as their players, and win based on their players running miles and times for the week.

**To Learn:** This project will help me learn the Rust language better and may use this project or an extension of this project to explore WebAssembly.

## Installation

Clone the repo and cd into the repo:

```shell
git clone https://github.com/kgalang/fantasyrustball.git
cd fantasyrustball
```

Copy over the example .env file:

```shell
cp .env.example .env
```

**IMPORTANT:** Change .env values for your setup, paying special attention to the salt and various keys.

After you set the `DATABASE` value in .env, you'll need it to match the `default` value in the `features` section in `Cargo.toml` with the `DATABASE` value in .env:

```toml
[features]
cockroach = []
mysql = []
postgres = []
sqlite = []
default = ["postgres"]
```

_note:_ Only supply a SINGLE database in the `default` array.

Next, you'll need to install the Diesel CLI:

```shell
cargo install diesel_cli
```

If you run into errors, see http://diesel.rs/guides/getting-started/

## Running the Database

You can run whatever database you'd like but this project has been set up for Postgres. One option to run locally is via `docker-compose`

```shell
docker-compose up db
```

Now run the migrations via the Diesel CLI:

```shell
diesel migration run
```

## Running the Server

To startup the server:

```shell
cargo run
```

## Autoreloading

Prerequisite: Make sure you have `systemfd` and `cargo-watch` installed. If not run:

```shell
cargo install systemfd
cargo install cargo-watch
```

To startup the server and autoreload on code changes:

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

### Running Tests

To run all of the tests:

```shell
cargo test
```

For an example to run all tests with `leagues` in the test name with printed statements:

```shell
cargo test leagues -- --nocapture
```

## Docker

To build a Docker image of the application:

```shell
docker build -t fantasyrustball .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name fantasyrustball fantasyrustball
```

## Non-Blocking Diesel Database Operations

When accessing a database via Diesel, operations block the main server thread.
This blocking can be mitigated by running the blocking code in a thread pool from within the handler.

Example:

```rust
pub async fn get_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, *user_id)).await?;
    respond_json(user)
}
```

Blocking errors are automatically converted into ApiErrors to keep the api simple:

```rust
impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError("Thread blocking error".into()),
        }
    }
}
```

## Credit

This project was built off of many great open source contributions including a Rust Actix example template here:

https://github.com/ddimaria/rust-actix-example

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
