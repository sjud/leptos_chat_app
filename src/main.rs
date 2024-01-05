cfg_if::cfg_if! {
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
            let     a =             00;
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
