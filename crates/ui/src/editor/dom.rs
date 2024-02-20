use std::collections::HashMap;

use freya::torin::{dom_adapter::DOMAdapter, node::Node};

#[derive(Clone)]
pub struct EditorLine {
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub node: Node,
    pub height: u16,
}

#[derive(Default)]
pub struct EditorDom {
    pub lines: HashMap<usize, EditorLine>,
}

impl EditorDom {
    pub fn add(
        &mut self,
        line_id: usize,
        children: Vec<usize>,
        line: Node,
        line_height: u16,
        parent: Option<usize>,
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

impl DOMAdapter<usize> for EditorDom {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
        self.lines
            .get(node_id)
            .map(|c| c.children.clone())
            .unwrap_or_default()
    }
    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.lines.get(node_id).map(|c| c.node.clone())
    }
    fn height(&self, node_id: &usize) -> Option<u16> {
        self.lines.get(node_id).map(|c| c.height)
    }
    fn is_node_valid(&mut self, _: &usize) -> bool {
        true
    }
    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.lines.get(node_id).and_then(|c| c.parent)
    }
    fn closest_common_parent(&self, _: &usize, _: &usize) -> Option<usize> {
        None
    }
}
