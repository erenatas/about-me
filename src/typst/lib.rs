use derive_typst_intoval::{IntoDict, IntoValue};
use typst::foundations::Bytes;
use typst::foundations::{Dict, IntoValue};
use typst::{layout::Abs, model::Document, text::Font};
use typst_as_lib::TypstTemplate;

pub struct TypstBuilder {
    typst_doc: Document,
}

impl TypstBuilder {
    pub fn new(file_data: &str, font_data: &[u8]) -> TypstBuilder {
        let font: Font = Font::new(Bytes::from(font_data), 0).expect("Could not parse font!");
        let template: TypstTemplate = TypstTemplate::new(vec![font], file_data);
        let doc: Document = template
            .compile_with_input(dummy_data())
            .output
            .expect("typst::compile() returned an error!");
        TypstBuilder { typst_doc: doc }
    }

    pub fn generate_svg(&mut self) -> String {
        typst_svg::svg_merged(&self.typst_doc, Abs::pt(2.0))
    }
}

// Some dummy content. We use `derive_typst_intoval` to easily
// create `Dict`s from structs by deriving `IntoDict`;
fn dummy_data() -> Content {
    Content { v: vec![] }
}

// Add this implementation
impl From<Content> for Dict {
    fn from(value: Content) -> Self {
        value.into_dict()
    }
}

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct Content {
    v: Vec<ContentElement>,
}

#[derive(Debug, Clone, Default, IntoValue, IntoDict)]
struct ContentElement {
    heading: String,
    text: Option<String>,
    num1: i32,
    num2: Option<i32>,
    image: Option<Bytes>,
}
