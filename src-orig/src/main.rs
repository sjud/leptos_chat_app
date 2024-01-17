cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        use tower_http::cors::{CorsLayer};
        use axum::{
            Router,
            routing::get,
            extract::State,
            http::Request,
            body::Body,
            response::IntoResponse
        };
        use leptos::{*,provide_context, LeptosOptions};
        use sqlx::SqlitePool;
        use leptos_axum::LeptosRoutes;
        use leptos_chat_app::fallback::file_and_error_handler;

        #[derive(Clone,Debug,axum_macros::FromRef)]
        pub struct ServerState{
            pub conn:SqlitePool,
            pub options:LeptosOptions,
            pub routes: Vec<leptos_router::RouteListing>,
        }

        pub async fn server_fn_handler(
            State(state): State<ServerState>,
            request: Request<Body>,
        ) -> impl IntoResponse {
            leptos_axum::handle_server_fns_with_context(
                move || {
                    provide_context(state.clone());
                },
                request,
            )
            .await
            .into_response()
        }

        pub async fn leptos_routes_handler(
            State(state): State<ServerState>,
            req: Request<Body>,
        ) -> axum::response::Response {
            let handler = leptos_axum::render_route_with_context(
                state.options.clone(),
                state.routes.clone(),
                move || {
                    provide_context(state.conn.clone());
                },
                leptos_chat_app::App,
            );
            handler(req).await.into_response()
        }

        #[tokio::main]
        async fn main() {
            let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.expect("db pool to work");

            sqlx::migrate!("../migrations")
                        .run(&pool)
                        .await
                        .expect("could not run SQLx migrations");

            let conf = get_configuration(Some("./src-orig/Cargo.toml")).await.unwrap();

            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes =  leptos_axum::generate_route_list(leptos_chat_app::App);

            let state = ServerState{
                options:leptos_options,
                conn:pool,
                routes:routes.clone(),
            };

            let cors = CorsLayer::new()
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_origin("tauri://localhost".parse::<axum::http::HeaderValue>().unwrap())
                .allow_headers(vec![axum::http::header::CONTENT_TYPE]);

            
            let app = Router::new()
                .route("/api/*fn_name",get(server_fn_handler).post(server_fn_handler))
                .layer(cors)
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(file_and_error_handler)
                .with_state(state);

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            logging::log!("listening on http://{}", &addr);
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
        }
    } else if #[cfg(feature="csr")]{
        pub fn main() {
            server_fn::client::set_server_url("http://127.0.0.1:3000");
            leptos::mount_to_body(leptos_chat_app::App);
        }
    } else {
        pub fn main() {

        }
    }
}
