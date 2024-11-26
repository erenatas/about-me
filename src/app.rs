use leptos::*;
use leptos_meta::*; 

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/about_me.css"/>
        <section class="about-section">
            <header class="header">
                <nav class="nav">
                    <div class="logo">"▽"</div>
                    <div class="nav-links">
                        <a href="https://github.com/erenatas">Github</a>
                        <a href="/blog">Blog</a>
                        <a href="/resume">Resume</a>
                    </div>
                </nav>
            </header>

            <div class="about-content">
                <h1 class="about-title"><span class="accent-dot">"->"</span>" about-me"</h1>

                <div class="developer-info">
                    <h2 class="developer-name">"Eren Atas"</h2>
                    <h3 class="developer-tagline">
                        "A site reliability engineer who loves tinkering"
                        <span class="accent-dot">.</span>
                        <span class="glitch-text" title="999">999</span>
                    </h3>
                </div>
            </div>
        </section>
    }
}
