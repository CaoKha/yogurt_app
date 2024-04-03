#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::middleware;
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_cookies::CookieManagerLayer;
    // use tracing_subscriber::EnvFilter;
    use yogurt_web::backend::model::ModelController;
    use yogurt_web::fileserv::file_and_error_handler;
    use yogurt_web::{app::*, backend};
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment

    tracing_subscriber::fmt()
        // .without_time()
        .with_target(false)
        // .with_env_filter(EnvFilter::from_default_env())
        .with_env_filter("yogurt_web=debug")
        .init();
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let mc = ModelController::new().await.unwrap();
    let route_apis = backend::routes::routes_tickets::routes(mc.clone()).route_layer(
        middleware::from_fn(backend::middleware::mw_auth::mw_require_auth),
    );

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .merge(backend::routes::routes_login::routes())
        .nest("/api", route_apis)
        .layer(middleware::map_response(
            backend::middleware::mw_response_mapper::response_mapper,
        ))
        .layer(middleware::from_fn_with_state(
            mc,
            backend::middleware::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());
    // .fallback_service(backend::routes::routes_static::routes());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
