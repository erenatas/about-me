pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

pub mod observability;
pub mod resume;
pub mod typst;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
