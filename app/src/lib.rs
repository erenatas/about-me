pub mod app;

#[cfg(feature = "ssr")]
pub mod app_state;
pub mod blog;
#[cfg(feature = "ssr")]
pub mod db;
pub mod observability;
pub mod resume;
#[cfg(feature = "ssr")]
pub mod service;
pub mod typst;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
