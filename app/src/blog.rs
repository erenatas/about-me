#[cfg(feature = "ssr")]
use crate::app_state::AppState;
#[cfg(feature = "ssr")]
use crate::service::query::Query as PostQuery;
use entity::post::Model;
use leptos::prelude::ElementChild;
use leptos::prelude::InnerHtmlAttribute;
use leptos::prelude::With;
use leptos::prelude::{ClassAttribute, Get, Resource, ServerFnError};
use leptos::*;
use leptos_router::hooks::use_params_map;
use leptos_router::nested_router::Outlet;
use prelude::{For, Suspense};

#[component]
pub fn BlogList() -> impl IntoView {
    let posts = Resource::new(|| (), |_| async move { get_blog_posts().await });

    view! {
      <div class="blog-container">
        <div class="blog-header">
          <span>date</span>
          <span>title</span>
          <span class="views">views</span>
        </div>
        <Suspense fallback=move || {
          view! { <p>"Loading..."</p> }
        }>
          {move || {
            posts
              .get()
              .and_then(|result| result.ok())
              .map(|posts| {
                view! {
                  <For each=move || posts.clone() key=|post| post.id children=move |post| view! { <BlogPost post /> } />
                }
              })
          }}
        </Suspense>
      </div>
    }
}

#[component]
pub fn BlogPost(post: Model) -> impl IntoView {
    view! {
      <div class="blog-post">
        <span class="date">{post.publication_date.format("%Y-%M").to_string()}</span>
        <a class="title" href=format!("/blog/{}", post.id.to_string())>
          {post.title}
        </a>
        <span class="views">{post.views.to_string()}</span>
      </div>
    }
}

#[component]
pub fn BlogContent() -> impl IntoView {
    let params = use_params_map();
    let post = Resource::new(
        move || {
            params
                .with(|p| p.get("id").map(|id| id.parse::<i32>().ok()))
                .flatten()
        },
        |id| async move {
            match id {
                Some(id) => find_post_by_id(id).await,
                None => Err(ServerFnError::new("No post ID provided")),
            }
        },
    );

    view! {
      <Suspense fallback=move || {
        view! { <p>"Loading post..."</p> }
      }>
        {move || {
          post
            .get()
            .map(|result| match result {
              Ok(maybe_post) => {
                match maybe_post {
                  Some(post) => {
                    view! {
                      <article class="blog-post-content">
                        <h1>{post.title}</h1>
                        <div class="post-meta">
                          <span>Eren Atas,{" "}</span>
                          <span class="date">{post.publication_date.format("%Y-%m-%d").to_string()}</span>
                        </div>
                        <div class="content" inner_html=post.content></div>
                      </article>
                    }
                  }
                  None => {
                    view! {
                      <article class="blog-post-content">
                        <h1>{"Not Found".to_string()}</h1>
                        <div class="post-meta">
                          <span>- 404{" "}</span>
                          <span class="date">{"-".to_string()}</span>
                        </div>
                        <div class="content" inner_html=Some("Post not found".to_string())></div>
                      </article>
                    }
                  }
                }
              }
              Err(_) => {
                view! {
                  <article class="blog-post-content">
                    <h1>{"Error".to_string()}</h1>
                    <div class="post-meta">
                      <span>Oh no,{""}</span>
                      <span class="date">{"-".to_string()}</span>
                    </div>
                    <div class="content" inner_html=Some("Error loading post".to_string())></div>
                  </article>
                }
              }
            })
        }}
      </Suspense>
    }
}

#[component]
pub fn Blog() -> impl IntoView {
    view! {
      <div class="blog-layout">
        <div class="blog-content">
          <Outlet />
        </div>
      </div>
    }
}

#[server]
pub async fn get_blog_posts() -> Result<Vec<Model>, ServerFnError> {
    use leptos::context::use_context;
    use leptos::prelude::ServerFnError;
    use opentelemetry::global;
    use opentelemetry::trace::Tracer;
    use tracing::info;

    let tracer: global::BoxedTracer = global::tracer("get_blog_posts");
    let state = use_context::<AppState>().expect("failed to find AxumState");

    tracer
        .in_span("call_db_list_posts", |_cx| async move {
            info!("calling find posts db");
            PostQuery::list_posts(&state.db)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))
        })
        .await
}

#[server]
pub async fn find_post_by_id(id: i32) -> Result<Option<Model>, ServerFnError> {
    use leptos::context::use_context;
    use leptos::prelude::ServerFnError;
    use opentelemetry::global;
    use opentelemetry::trace::Tracer;
    use tracing::{error, info};

    let tracer: global::BoxedTracer = global::tracer("get_blog_posts");
    let state = use_context::<AppState>().expect("failed to find AxumState");

    tracer
        .in_span("call_db_find_post_by_id", |_cx| async move {
            info!("calling find post by id db");
            let _ = PostQuery::increment_post_view(&state.db, id)
                .await
                .map_err(|e| {
                    error!("Failed to increment post views: {}", e);
                    ServerFnError::new(e.to_string())
                });
            PostQuery::find_post_by_id(&state.db, id)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))
        })
        .await
}
