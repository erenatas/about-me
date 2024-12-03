#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use about_me::observability::lib::get_axum_metrics_layer;
    use about_me::observability::metrics;
    use about_me::{app::*, observability};
    use about_me::fileserv::file_and_error_handler;
    use axum::Router;
    use axum::middleware;
    use axum::body::Body;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing::info;

    std::env::set_var("RUST_LOG", "info,warn,error");
    observability::lib::init_opentelemetry();
    let metrics_layer = get_axum_metrics_layer();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(metrics_layer)
        .layer(axum::middleware::from_fn(|req: axum::http::Request<Body>, next: middleware::Next| async move {
            metrics::API_REQUESTS.add(1, &[]);
            next.run(req).await
        }))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("listening on http://{}", &addr);

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
