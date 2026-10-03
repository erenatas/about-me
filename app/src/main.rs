#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use about_me::app_state::AppState;
    use about_me::db::DB;
    use about_me::app::*;
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::file_and_error_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use std::env;
    use tracing::info;

    std::env::set_var("RUST_LOG", "info,warn,error");
    // observability::lib::init_opentelemetry();  // disabled: no alloy
    // let _metrics_layer = get_axum_metrics_layer();  // disabled: no alloy
    // match init_pyroscope() {  // disabled: no pyroscope
    //     Ok(pyroscope) => {
    //         pyroscope.start().expect("Pyroscope failed to start");
    //         info!("Pyroscope started.")
    //     }
    //     Err(error) => {
    //         error!("Pyroscope failed to initialize: {}", error)
    //     }
    // };

    let db_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment variables");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let db = DB { conn: conn };

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf: ConfFile = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state: AppState = AppState {
        db: db,
        leptos_options: leptos_options,
    };

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let state = app_state.clone();
                move || provide_context(state.clone())
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

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
