use crate::db::DB;
use axum_macros::FromRef;
use leptos::config::LeptosOptions;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DB,
}
