use leptos::*;

#[component]
pub fn Resume() -> impl IntoView {
    let resume_data = create_local_resource(|| (), |_| async move { get_resume_svg().await });

    view! {
        <div>
            {move || match resume_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(Ok(data)) => view! { <div class="resume-svg" inner_html=data></div> }.into_view(),
                Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_view(),
            }}
        </div>
    }
}

#[server]
pub async fn get_resume_svg() -> Result<String, ServerFnError> {
    use crate::typst::lib::TypstBuilder;
    use opentelemetry::global;
    use opentelemetry::trace::Tracer;
    use tracing::info;

    let tracer: global::BoxedTracer = global::tracer("get_resume_svg");
    let out = tracer.in_span("call_api", |_cx| -> Result<String, ServerFnError> {
        info!("Generating resume..");
        let current_dir = std::env::current_dir().map_err(|e| ServerFnError::new(e))?;

        let font_path = current_dir
            .join("resources")
            .join("typst")
            .join("NewCMMath-Regular.otf")
            .display()
            .to_string();

        let file_path = current_dir
            .join("resources")
            .join("typst")
            .join("main.typ")
            .display()
            .to_string();

        let font_data = std::fs::read(&font_path).map_err(|e| ServerFnError::new(e))?;
        let file_data = std::fs::read_to_string(&file_path).map_err(|e| ServerFnError::new(e))?;

        let mut typst_builder = TypstBuilder::new(&file_data, &font_data);
        Ok(typst_builder.generate_svg())
    })?;

    Ok(out)
}
