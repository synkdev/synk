use std::collections::HashMap;

use freya::torin::node::Node;

#[derive(Clone)]
pub struct EditorLine {
    pub children: Vec<Node>,
    pub node: Node,
    pub height: f32,
}

#[derive(Default)]
pub struct EditorDom {
    pub lines: HashMap<usize, EditorLine>,
}

impl EditorDom {
    pub fn add(&mut self, line_id: usize, children: Vec<Node>, line: Node, line_height: f32) {
        self.lines.insert(
            line_id,
            EditorLine {
                children,
                height: line_height,
                node: line,
            },
        );
    }
}
