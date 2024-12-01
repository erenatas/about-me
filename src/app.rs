use crate::resume::Resume;
#[cfg(feature = "ssr")]
use axum::http::StatusCode;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Stylesheet id="leptos" href="/pkg/about_me.css"/>
            <section class="about-section">
                <header class="header">
                    <nav class="nav">
                        <a class="logo" href="/">"▽"</a>
                        <div class="nav-links">
                            <a href="https://github.com/erenatas">Github</a>
                            <a href="/blog">Blog</a>
                            <a href="/resume">Resume</a>
                        </div>
                    </nav>
                </header>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/resume" view=Resume/>
                    <Route path="/blog" view=UnderConstructionPage/>
                </Routes>
            </section>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="about-content">
            <h1 class="about-title"><span class="accent-dot">"->"</span>" about-me"</h1>

            <div class="developer-info">
                <h4 class="developer-name">"Eren Atas"</h4>
                <a class="developer-tagline">
                    "A site reliability engineer who loves tinkering"
                    <span class="accent-dot">.</span>
                    <span class="glitch-text" title="999">999</span>
                </a>
            </div>
        </div>
    }
}

#[component]
fn UnderConstructionPage() -> impl IntoView {
    // Server-side status code setting
    #[cfg(feature = "ssr")]
    {
        let response = use_context::<ResponseOptions>();
        if let Some(res) = response {
            res.set_status(StatusCode::IM_A_TEAPOT); // 418 status code
        }
    }

    view! {
        <div class="under-construction">
            <h1>"🚧 Under Construction 🚧"</h1>
            <img src="/418.png" width="40%"/>
            <p>"This page is currently brewing..."</p>
        </div>
    }
}
