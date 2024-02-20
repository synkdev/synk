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
        _node: &Node,
        _parent_area: &freya::prelude::Area,
        _available_parent_area: &freya::prelude::Area,
    ) -> Option<Size2D> {
        if let Some(node) = self.dom.nodes.get(&node_id) {
            match node.node_type {
                NodeType::Line => {}
                NodeType::Char => {}
            }
        }
        None
    }
}

#[derive(Clone)]
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
    pub node_type: NodeType,
}

#[derive(Default)]
pub struct EditorDom {
    pub nodes: HashMap<usize, EditorNode>,
}

impl EditorDom {
    pub fn add(
        &mut self,
        line_id: usize,
        children: Vec<usize>,
        line: Node,
        parent: Option<usize>,
        node_type: NodeType,
    ) {
        let parent_height = parent
            .map(|p| self.nodes.get(&p).unwrap().height)
            .unwrap_or(0);

        let height = parent_height + 1;
        self.nodes.insert(
            line_id,
            EditorNode {
                parent,
                children,
                height,
                node: line,
                node_type,
            },
        );
    }
}

impl DOMAdapter<usize> for EditorDom {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
        self.nodes
            .get(node_id)
            .map(|c| c.children.clone())
            .unwrap_or_default()
    }
    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.nodes.get(node_id).map(|c| c.node.clone())
    }
    fn height(&self, node_id: &usize) -> Option<u16> {
        self.nodes.get(node_id).map(|c| c.height)
    }
    fn is_node_valid(&mut self, _: &usize) -> bool {
        true
    }
    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.nodes.get(node_id).and_then(|c| c.parent)
    }
    fn closest_common_parent(&self, _: &usize, _: &usize) -> Option<usize> {
        None
    }
}
