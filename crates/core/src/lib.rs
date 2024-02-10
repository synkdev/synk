use crop::Rope;

pub struct Document {
    pub contents: Rope,
}

impl Document {
    pub fn new(initial_contents: String) -> Self {
        Document {
            contents: Rope::from(initial_contents),
        }
    }
}
