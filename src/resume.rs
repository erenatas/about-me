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
    use tracing::info;

    // Get the current directory and construct absolute paths
    info!("Generating resume..");
    let current_dir = std::env::current_dir()?;
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
    let font_data: Vec<u8> = std::fs::read(&font_path)?;
    let file_data = std::fs::read_to_string(&file_path)?;
    let mut typst_builder = TypstBuilder::new(&file_data, &font_data);

    let svg = typst_builder.generate_svg();

    Ok(svg)
}
