pub mod document;
pub mod highlighter;
pub mod default_themes;

use document::Document;

pub struct Core {
    pub documents: Vec<Document>,
}

impl Core {
    pub fn new() -> Self {
        Core {
            documents: vec![Document::new(
                "fn main() {
    println!(\"Hello world!\");
}"
                .to_string(),
            )],
        }
    }
}
