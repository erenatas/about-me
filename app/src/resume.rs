use leptos::prelude::ServerFnError;
use leptos::prelude::{ClassAttribute, ElementChild, InnerHtmlAttribute, Resource};
use leptos::*;
use prelude::{Get, Suspense};

#[component]
pub fn Resume() -> impl IntoView {
    let resume_async_data = Resource::new(|| (), |_| async move { get_resume_svg().await });

    view! {
      <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        <div>
          {move || match resume_async_data.get() {
            None => view! { <div class="resume-svg" inner_html="<p>Loading...</p>".to_string()></div> }.into_view(),
            Some(Ok(data)) => view! { <div class="resume-svg" inner_html=data></div> }.into_view(),
            Some(Err(e)) => {
              view! { <div class="resume-svg" inner_html=format!("<p>Error: {}</p>", e.to_string()).to_string()></div> }
                .into_view()
            }
          }}
        </div>
      </Suspense>
    }
}

#[server]
pub async fn get_resume_svg() -> Result<String, ServerFnError> {
    use crate::typst::lib::TypstBuilder;
    use opentelemetry::global;
    use opentelemetry::trace::Tracer;
    use tracing::{error, info};

    let tracer: global::BoxedTracer = global::tracer("get_resume_svg");
    let out = tracer.in_span("call_api", |_cx| -> Result<String, ServerFnError> {
        info!("Generating resume..");
        let current_dir = std::env::current_dir().map_err(|e| ServerFnError::new(e))?;

        let typst_dir_path: std::path::PathBuf =
            current_dir.join("app").join("resources").join("typst");

        let font_path = typst_dir_path
            .join("NewCMMath-Regular.otf")
            .display()
            .to_string();

        let file_path = typst_dir_path.join("main.typ").display().to_string();

        let font_data = std::fs::read(&font_path).map_err(|e| ServerFnError::new(e))?;
        let file_data = std::fs::read_to_string(&file_path).map_err(|e| ServerFnError::new(
          {
            error!("Error: {}, file: {}", e, &file_path);
            e
          }
        ))?;

        let mut typst_builder = TypstBuilder::new(&file_data, &font_data);
        Ok(typst_builder.generate_svg())
    })?;

    Ok(out)
}
