To start we want to pull in all our necessary cargo crates. Today we're making a full stack chat chat app using websockets, Axum and the Leptos frontend framework. So let's add the following to our Cargo.toml<br>
Optional files will only be for our server.

```toml
#We'll add axum, and specify `0.6.0` because leptos_axum uses that version.
axum = {version = "0.6.0", optional=true}
#axum-macros will help us by generating axum related code
axum-macros = { version= "0.3.0", optional=true}
#We'll use cfg_if to use different code for our server and our client.
cfg-if = "1.0.0"
#This helps us print logs into our console for our wasm client.
console_error_panic_hook = "0.1.7"
#This helps us print logs into our console for our wasm client.
console_log = "1.0.0"
leptos = "0.5.4"
#Let's add leptos_use  is an unofficial crate that has a web socket hook we'll use.
leptos-use = "0.9.0"
leptos_axum = {version = "0.5.4",optional=true}
leptos_meta = "0.5.4"
leptos_router = "0.5.4"
#we'll use sqlx to persist messages and remember users in a sqlite database.
sqlx = { version = "0.7.3", optional = true, features=["sqlite","runtime-tokio"]}
#It's an async rust app so let's add tokio. We'll need it to run our server and for sqlx.
tokio = { version = "1.35.1", optional=true }
tower = {version = "0.4.10", optional = true, featyres=["util"]}
#We'll use tower_http to serve our favicon. tower_http is a middleware library for axum.
tower-http = { version = "0.3.4", optional = true, features=["fs"]}
#We'll need wasm_bindgen to call our wasm file from javascript. But we won't be writing any javascript. That'll happen at build time #with cargo leptos. But we need to write the rust glue that will be called.
wasm-bindgen = "0.2.89"
```

And let's copy and paste the following at the bottom of our Cargo.toml since it's a Leptos project. Leptos apps are split into the server side which we call "ssr" and the client side which is our "hydrate" feature. The `[lib]` crate-type is for cargo leptos, we can't build the project without it. 
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["ssr"] # gets up autocomplete in visual studio code.
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr = [
    "dep:axum",
    "leptos/ssr",
    "leptos-use/ssr",
    "dep:leptos_axum",
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "dep:tower-http",
    "dep:sqlx",
    "dep:tokio",
]
```
These are our leptos config environment variables they'll show up later in our main.rs as LeptosOptions, but they'll also be used by 
cargo leptos when we build our app.
```toml
[package.metadata.leptos]
output-name = "leptos_chat_app"
site-root = "target/site"
site-addr = "0.0.0.0:3000"
reload-port = 3001
browserquery = "defaults"
watch = false
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```
And let's add a dev-dependency section which we'll use when building with end to end testing.
```toml
[dev-dependencies]
cucumber = "0.19.1"
fantoccini = "0.19.3"
```
So our full Cargo.toml will look like this
```toml
[package]
name = "leptos_chat_app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
axum = {version = "0.6.0", optional=true}
axum-macros = { version= "0.3.0", optional=true}
cfg-if = "1.0.0"
console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
leptos = "0.5.4"
leptos-use = "0.9.0"
leptos_axum = {version = "0.5.4",optional=true}
leptos_meta = "0.5.4"
leptos_router = "0.5.4"
sqlx = { version = "0.7.3", optional = true, features=["sqlite","runtime-tokio"]}
tokio = { version = "1.35.1", optional=true }
tower = {version = "0.4.10", optional = true, featyres=["util"]}
tower-http = { version = "0.3.4", optional = true, features=["fs"]}
wasm-bindgen = "0.2.89"

[dev-dependencies]
cucumber = "0.19.1"
fantoccini = "0.19.3"

[features]
default = ["ssr"] 
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr = [
    "dep:axum",
    "dep:axum-macros",
    "leptos/ssr",
    "leptos-use/ssr",
    "dep:leptos_axum",
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "dep:tower-http",
    "dep:tower",
    "dep:sqlx",
    "dep:tokio",
]



[package.metadata.leptos]
output-name = "leptos_chat_app"
site-root = "target/site"
site-addr = "0.0.0.0:3000"
reload-port = 3001
browserquery = "defaults"
watch = false
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```

One possible hang up when defining our Cargo.toml. We use `leptos_axum 5.4` which relies on `axum 0.6.0` we also call
tower_http to send our favicon. The latest version of axum is `0.7.0` and the latest version of tower_http is `0.5.0`.
If you specify the wrong version of axum or of tower_http, you're going to get very confusing error messages. There are breaking changes across versions, and when one library relies ontop of another one you're going to get type mismatch errors. But it won't be clear why they mismatch, and it will look like you are using the libraries together in the wrong way. Usually the error messages will tell you something like
```rust
    = note: `Response<UnsyncBoxBody<Bytes, Error>>` and `Response<UnsyncBoxBody<Bytes, Error>>` have similar names, but are actually distinct types
```
But sometimes they will not, particularly if you are dealing with traits.
```rust
error[E0277]: the trait bound `ServeFileSystemResponseBody: HttpBody` is not satisfied
   --> src/main.rs:100:27
    |
100 |                 .map(|res|res.map(axum::body::boxed))
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `HttpBody` is not implemented for `ServeFileSystemResponseBody`
```
So if you are bringing in traits across multiple crates, and using an old version of atleast one crate and you find that things are not working as shown in a repo's examples make sure all of your libraries versions are expecting each other.<br>
In this case, we need `tower_http 0.3.4`.<br>

Let's build the folder structure of our project.<br>
A `migrations` folder for our sqlite migrations.<br>
A `tests` folder for our tests<br>
A `features` folder for our cucumber features, we'll cover this later when we write our first end to end test.<br>
A `public` folder for our static files.<br>
A `.github/workflows` folders for our github files and one to store our workflows.
```sh
mkdir migrations && mkdir tests && mkdir features && mkdir public && mkdir .github && mkdir .github/workflows
```
And inside of our srcs folder we're going to organize our app in the following way.<br>
We're splitting the main file into the code for the server and the code for the client. <br>

So our wasm app will only have a very brief main function at the end of the file to help trunk build our wasm app.<br>
Cargo leptos uses trunk when building our wasm frontend.
```rust
//main.rs
cfg_if::cfg_if!{
    if #[cfg(feature="ssr")]
    {
        use axum::{
            Router, 
            routing::get,
            extract::{Path, RawQuery, State}, 
            http::HeaderMap,
            http::Request, 
            body::Body, 
            response::IntoResponse
        };
        use std::collections::HashMap;
        use leptos::{*,provide_context, LeptosOptions};
        use sqlx::SqlitePool;
        use leptos_axum::LeptosRoutes;

        #[derive(Clone,Debug,axum_macros::FromRef)]
        pub struct ServerState{
            conn:SqlitePool,
            options:LeptosOptions,
        }
  
        #[tokio::main]
        async fn main() {
            let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.expect("db pool to work");
        
            sqlx::migrate!()
                        .run(&pool)
                        .await
                        .expect("could not run SQLx migrations");
        
            let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
        
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes =  leptos_axum::generate_route_list(leptos_chat_app::App);
            
            let state = ServerState{
                options:leptos_options,
                conn:pool,
            };
            
            let app = Router::new()
                .route("/favicon.ico", get(favicon))
                .route("/api/*fn_name",get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(||async{})
                .with_state(state);
        
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap()
        }
        
        pub async fn server_fn_handler(
            State(state): State<ServerState>,
            path: Path<String>,
            headers: HeaderMap,
            raw_query: RawQuery,
            request: Request<Body>,
        ) -> impl IntoResponse {
            leptos_axum::handle_server_fns_with_context(
                path,
                headers,
                raw_query,
                move || {
                    provide_context(state.clone());
                },
                request,
            )
            .await
            .into_response()
        }

        pub async fn leptos_routes_handler(
            Path(_params): Path<HashMap<String, String>>,
            State(state): State<ServerState>,
            axum::extract::State(option): axum::extract::State<LeptosOptions>,
            request: Request<Body>,
        ) -> axum::response::Response {
            
            let handler = leptos_axum::render_app_async_with_context(
                option.clone(),
                move || {
                    provide_context(state.clone());
                },
                move || view! {  <leptos_chat_app::App/> },
            );
    
            handler(request).await.into_response()
        }

        async fn favicon() -> Result<axum::http::Response<axum::body::BoxBody>,axum::http::StatusCode> {
            use tower::ServiceExt;
            tower_http::services::ServeFile::new("favicon.ico")
                .oneshot(axum::http::Request::builder().body(()).unwrap())
                .await
                .map(|res|res.map(axum::body::boxed))
                .map_err(|_|axum::http::StatusCode::NOT_FOUND)
        }
    } else {
        pub fn main() {
            use leptos_chat_app::*;
            use leptos::*;
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(|| {
                view! {  <App/> }
            });
        }
    }
}
```

cfg_if let's us use if statements with cfg macros. So we can more easily split our app up into server and client.
```rust
cfg_if::cfg_if!{
    if #[cfg(feature="ssr")]
    //...
}
```

Let's look at our first connection to an outside service, this is for our database.
```rust
let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.expect("db pool to work");
```
 We'll use sqlx to establish a connection to an sqlite database in our servers memory. <br>
 
 This will reset each time we restart our server, which will make prototyping easier as we'll just change our migration files and rerun our server for a fresh database each time.<br>

This creates a pool, which is a bunch of possible connections that are used by our server as it handles requests asyncronously.<br>
We don't need to know all of the details, but we do need to know how Axum and Sqlx interact. <br>
We have to answer: How do we pass it into our server? How will our server handle it? And the implications of these questions
are is it "async ready", and is it cheap to clone? If it's not ready to be used in async code, we might have to wrap it in `Arc<T>`. Axum uses clone to pass state into requests, if it's not cheap to clone that will be a big problem for our performance.<br>

Let's look at the code for Pool
```rust
pub struct Pool<DB: Database>(pub(crate) Arc<PoolInner<DB>>);
```
It's just a wrapper around `Arc<PoolInner<DB>>` and in our case `Arc<PoolInner<SqLite>>` 

`Arc<Wrapper<Arc<T>>>` is redunant so let's move on, but before we do. Let's look at PoolInner's signature
```rust
pub(crate) struct PoolInner<DB: Database>
```

`PoolInner` is a `pub(crate)` type inside of sqlx, which means we don't actually access it. We only use it through the sqlx API. <br>

Per the Pool docs<br>
"Pool is Send, Sync and Clone. It is intended to be created once at the start of your application/daemon/web server/etc. and then shared with all tasks throughout the process’ lifetime. How best to accomplish this depends on your program architecture...
Cloning Pool is cheap as it is simply a reference-counted handle to the inner pool state..."<br>     
So we've answered our questions, it's async ready and it's cheap to clone.<br>
We're going to create it at the start of our main.rs and then we're just going to pass it into our server with a `with_state` call on the router.<br>
 

Moving on<br>
This is our migration code, when our server starts we'll <i>migrate</i> which will look inside of the migrations folder we created earlier.
We're going to put some sqlite files in there and it will run them against the empty sqlite database we created in memory.
```rust
sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");
```

Earlier we added a bunch of data in our Cargo.toml underneath `[package.metadata.leptos]` and this is one of the things that what we do with it.
    [get_configuration](https://docs.rs/leptos/latest/leptos/fn.get_configuration.html) churns out a configuration file for our leptos app. Which has one field [LeptosOptions](https://docs.rs/leptos_config/0.5.4/leptos_config/struct.LeptosOptions.html)

```rust
let conf = leptos::get_configuration(Some("Cargo.toml")).await.unwrap();
```
generate_route_list  looks into our app finds the Router component
and builds a set of axum routes that matches our Router component. <br>
The leptos_routes_handler will render our app and route it based on the routes we'll describe in our App's Router component. It takes leptos options because it needs them to build the response it sends to the client.<br> 
We're getting out `ServerState` and providing it as a context into our app, so when we build the app we'll be able to query
the database as we render by using leptos's `use_context` functions.
One last thing, we're using render_app_async_with_context, this means we'll only start sending the html to the user after async resources have loaded. So if we query a database to fill in information on the page we'll wait for the result before sending it to the user. 
```rust
let leptos_options = conf.leptos_options;
let routes =  leptos_axum::generate_route_list(leptos_chat_app::App);
pub async fn leptos_routes_handler(
            Path(_params): Path<HashMap<String, String>>,
            State(state): State<ServerState>,
            axum::extract::State(option): axum::extract::State<LeptosOptions>,
            request: Request<Body>,
        ) -> axum::response::Response {
            
            let handler = leptos_axum::render_app_async_with_context(
                option.clone(),
                move || {
                    provide_context(state.clone());
                },
                move || view! {  <leptos_chat_app::App/> },
            );
    
            handler(request).await.into_response()
        }
``` 
We'll put these two together in our axum Router call, that we use when building the server. Not to be confused with the Leptos Router.


```rust
let app = Router::new()
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
```
Let's add a favicon to our public folder https://raw.githubusercontent.com/leptos-rs/leptos/main/examples/todo_app_sqlite_axum/public/favicon.ico <br>
Anything in our public folder will be served from the root of our app because we added the line `assets-dir = "public"`
beneath `[package.metadata.leptos]` so everything in our public folder will be copied into the folder our server runs from.<br>
```rust
    .route("/favicon.ico", get(tower_http::services::ServeFile::new("favicon.ico")))
```
When we write server functions, they are automatically registered with our app all we need is our router to have this route and handler. Server functions automatically get their routes names. We provide the state as context for the server functions so instead of calling `State(server_state):State<ServerState>` in our server functions we use `use_context`.
```rust
    .route("/api/*fn_name",get(leptos_axum::handle_server_fns).post(leptos_axum::handle_server_fns))

    pub async fn server_fn_handler(
            State(state): State<ServerState>,
            path: Path<String>,
            headers: HeaderMap,
            raw_query: RawQuery,
            request: Request<Body>,
        ) -> impl IntoResponse {
            leptos_axum::handle_server_fns_with_context(
                path,
                headers,
                raw_query,
                move || {
                    provide_context(state.clone());
                },
                request,
            )
            .await
            .into_response()
        }
```
Let's think about how to handle fallbacks when we start writing our leptos app. Let's keep it empty for now.<br>
When no routes match we serve a fallback. That means, no routes inside of our leptos Component Router, and no routes inside of our axum Router.
```rust
    .fallback(||async{
        // Empty.
    })
```
Finally add state to our router, this is the pool we started earlier.<br>
We've been talking about ServerState but we haven't talked about it until now. It's just a struct that will hold onto
any of the services we want to call inside of our app. We could add reqwest clients, database connections, channel recievers
and senders or more. But this is cloned everytime we handle a new request so we need to be aware of how the items we pass into it
are cloned as well as how they behave across threads.
```rust
            let state = ServerState{
                options:leptos_options,
                conn:pool,
            };

        .with_state(state);
```
Our final router will look like this
```rust
    let app = Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/api/*fn_name",get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(||async{})
        .with_state(state);
```
In our Cargo.toml file we had this line under `[package.metadata.leptos]``
```
site-addr = "0.0.0.0:3000"
```

That gets read into our leptos options.<br>
We'll bind on the address and we turn our Router into a service, which is standard Axum stuff.
```rust
    let addr = leptos_options.site_addr;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
```
And we'll be able to access our server on the local host 127.0.0.1:3000

Let's look at our lib.rs file.
```rust
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {}
}

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            #[cfg(debug_assertions)]
            console_error_panic_hook::set_once();
            leptos::mount_to_body(App);
        }
    }
}
```

And our app component, remember that we read the Router component to find the routes of our app? We don't have a Router Component yet.
We'll add one one soon when we start wrting Routes for our app!

```rust
#[component]
pub fn App() -> impl IntoView {
    view! {}
}
```

This gets called by the JS script that's generated in the build process.<br>
The wasm can't actually run itself without JS first calling into it.<br>
That's why the `#[wasm_bindgen]` macro is applied.<br>

```rust
cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            #[cfg(debug_assertions)]
            console_error_panic_hook::set_once();
            leptos::mount_to_body(App);
        }
    }
}
```

Let's look at the Dockerfile for deploying to Digital Ocean
```Dockerfile
FROM rust:buster
RUN rustup override set nightly && rustup target add wasm32-unknown-unknown && apt install binaryen
RUN cargo install --features no_downloads --locked cargo-leptos
RUN cargo leptos build --release --bin-features ssr
EXPOSE 3000
ENTRYPOINT ["./target/release/leptos_chat_app"]
```
It's pretty simple, we use the Rust base image. We set rustup to override nightly (because we want to use leptos nightly features),
we need to add the wasm32-unkown-unknown compilation target so cargo leptos build can build the client, and we use apt (linux package mangement tool) to install binaryen which is toolchain for WASM. We need to EXPOSE the port that we set in our Cargo.toml under site-addr. And, while we're at it let's add a profile realease optimization to our Cargo.toml

```toml
[profile.release]
codegen-units = 1
lto = true
```
Codegen units are parallel optimizations the rust compiler performs, telling it to use only 1 or in otherwords not to parralelize
compilation can increase it's ability to optimize the code itself. LTO, link time optimization will try to optimize across crate boundries such as our dependencies. But, it makes compiling even slower.

While we're thinking about release optimzations lets create a index.html file.

```html
<!DOCTYPE html>
<html>
	<head>
		<link data-trunk rel="rust" data-wasm-opt="z"/>
		<link data-trunk rel="icon" type="image/ico" href="/favicon.ico"/>
	</head>
	<body></body>
</html>
```
This line
```html
		<link data-trunk rel="rust" data-wasm-opt="z"/>
```
Tells trunk to use wasm optimization 'z' which is the one that will reduce final binary size. The wasm binary is the thing we send
to the client, so getting it as small as possible is one of our goals. We're prioritizing a fast server and a small client. 

When we get Digital Ocean set up, we'll track the main branch and when we push our code to the main branch, Digital Ocean will see that and redeploy. Let's set up a CICD workflow to make sure the code we merge onto our github main branch passes our tests.<br>

Let's create a standard Clippy, cargo fmt, cargo test workflow. That has three jobs, the first runs checks cargo fmt on our code, the second
checks to see if we getting any clippy warnings and the third that runs our tests. Each job will run in parallel.

```yaml
name: Rust Code Checks
run-name: ${{ github.actor }} Push Code
on: [push]

jobs:
  format:
    name: Format Rust code
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - run: rustup component add rustfmt
      - run: cargo fmt -- --check

  clippy:
    name: Lint with Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy
          override: true
      - run: cargo clippy -- -D warnings

  test:
    name: Run tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - run: cargo test
```

Now this doesn't automatically prevent failures from being pushed to do that we need to create a branch protection rule in github.<br>
Go to your GitHub repository.<br>
Click on "Settings" at the top.<br>
In the left sidebar, click "Branches."<br>
Under "Branch protection rules," you can view existing rules or add new ones.<br>
We'll set branch name pattern to '*' which is wildcard and applies it to all branches.<br>
We'll require status check before merging and add `Format Rust Code`, `Lint With Clippy`, and `Run Tests`. <br>
And set it so that we do not allow settings to be bypassed, so that when I push to the repo as the admin it will still gatekeep my code. <br>
