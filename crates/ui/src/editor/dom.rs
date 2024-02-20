use std::collections::HashMap;

use freya::torin::{dom_adapter::DOMAdapter, node::Node};

#[derive(Clone)]
pub struct EditorLine {
    pub parent: Option<Node>,
    pub children: Vec<Node>,
    pub node: Node,
    pub height: f32,
}

#[derive(Default)]
pub struct EditorDom {
    pub lines: HashMap<usize, EditorLine>,
}

impl EditorDom {
    #[inline]
    pub fn add(
        &mut self,
        line_id: usize,
        children: Vec<Node>,
        line: Node,
        line_height: f32,
        parent: Option<Node>,
    ) {
        self.lines.insert(
            line_id,
            EditorLine {
                parent,
                children,
                height: line_height,
                node: line,
            },
        );
    }
}
