pub mod document;

use std::path::PathBuf;

use document::Document;

pub struct Core {
    pub documents: Vec<Document>,
}

impl Core {
    pub fn new() -> Self {
        Core {
            documents: vec![Document::from_file(PathBuf::from("./document.rs"))],
        }
    }
}
