use fscx_rs::file::read_to_string;
use std::path::PathBuf;

use ropey::Rope;

#[derive(Clone, PartialEq)]
pub struct Document {
    pub rope: Rope,
}

impl Document {
    pub fn new(initial_contents: String) -> Self {
        let rope = Rope::from(initial_contents);
        Document { rope }
    }

    pub fn from_file(file: PathBuf) -> Self {
        let file = read_to_string(file).unwrap();
        Self::new(file)
    }
}
