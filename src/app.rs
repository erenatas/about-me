use crate::{observability::metrics, resume::Resume};
#[cfg(feature = "ssr")]
use axum::http::StatusCode;
use leptos::*;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/about_me.css"/>

        // sets the document title
        <Title text="Eren Atas"/>

        // content for this welcome page
        <Router>
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
                    <Routes fallback=|| NotFoundPage>
                        <Route path=path!("/") view=HomePage/>
                        <Route 
                            path=path!("/resume") 
                            view=Resume
                            ssr=leptos_router::SsrMode::PartiallyBlocked
                        />
                        <Route path=path!("/blog") view=UnderConstructionPage/>
                    </Routes>
                </section>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    metrics::PAGE_VIEWS.add(1, &[]);
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
        <div class="response-code-kawaiilogos">
            <h1>"🚧 Under Construction 🚧"</h1>
            <img src="/418.png" width="40%"/>
            <p>"This page is currently brewing..."</p>
        </div>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="response-code-kawaiilogos">
            <img src="/404.png" width="40%"/>
        </div>
    }
}