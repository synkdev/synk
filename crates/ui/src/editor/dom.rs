use std::collections::HashMap;

use freya::torin::{
    custom_measurer::LayoutMeasurer, dom_adapter::DOMAdapter, geometry::Size2D, node::Node,
};

pub struct TextMeasurer {
    dom: EditorDom,
}

impl LayoutMeasurer<usize> for TextMeasurer {
    fn measure(
        &mut self,
        node_id: usize,
        node: &Node,
        parent_area: &freya::prelude::Area,
        available_parent_area: &freya::prelude::Area,
    ) -> Option<Size2D> {
        if let Some(line) = self.dom.get_node(&node_id) {}
        None
    }
}

pub enum NodeType {
    Line,
    Char,
}

#[derive(Clone)]
pub struct EditorNode {
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub node: Node,
    pub height: u16,
}

#[derive(Default)]
pub struct EditorDom {
    pub lines: HashMap<usize, EditorNode>,
}

impl EditorDom {
    pub fn add(&mut self, line_id: usize, children: Vec<usize>, line: Node, parent: Option<usize>) {
        let parent_height = parent
            .map(|p| self.lines.get(&p).unwrap().height)
            .unwrap_or(0);

        let height = parent_height + 1;
        self.lines.insert(
            line_id,
            EditorNode {
                parent,
                children,
                height,
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
