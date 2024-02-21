use std::collections::HashMap;

use freya::{
    hooks::Line,
    torin::{
        custom_measurer::LayoutMeasurer, dom_adapter::DOMAdapter, geometry::Size2D, node::Node,
    },
};
use ropey::RopeSlice;
use skia_safe::{Font, Paint};

pub struct TextMeasurer<'a> {
    pub dom: EditorDom,
    pub font: &'a Font,
    pub paint: &'a Paint,
}

impl<'a> LayoutMeasurer<usize> for TextMeasurer<'a> {
    fn measure(
        &mut self,
        node_id: usize,
        _node: &Node,
        _parent_area: &freya::prelude::Area,
        _available_parent_area: &freya::prelude::Area,
    ) -> Option<Size2D> {
        if let Some(node) = self.dom.nodes.get(&node_id) {
            match &node.node_type {
                NodeType::Line { chars } => {
                    let mut line_len = Size2D::zero();
                    for char in chars {
                        let measure = measure_char(*char, &self.font, &self.paint);
                        line_len += measure
                    }
                    return Some(line_len);
                }
                NodeType::Char(char) => return Some(measure_char(*char, &self.font, &self.paint)),
                NodeType::Root => return None,
            }
        }
        None
    }
}

pub fn measure_char(char: char, font: &Font, paint: &Paint) -> Size2D {
    let text_bounds = font.measure_text(char.to_string().as_str(), Some(paint)).1;
    Size2D::new(text_bounds.width(), text_bounds.height())
}

#[derive(Clone)]
pub struct LineChar {
    pub char: char,
    pub font: Font,
}

#[derive(Clone)]
pub enum NodeType {
    Line { chars: Vec<char> },
    Char(char),
    Root,
}

#[derive(Clone)]
pub struct EditorNode {
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub node: Node,
    pub height: u16,
    pub node_type: NodeType,
}

#[derive(Default, Clone)]
pub struct EditorDom {
    pub nodes: HashMap<usize, EditorNode>,
}

impl EditorDom {
    pub fn add(
        &mut self,
        id: usize,
        children: Vec<usize>,
        node: Node,
        parent: Option<usize>,
        node_type: NodeType,
    ) {
        let parent_height = parent
            .map(|p| self.nodes.get(&p).unwrap().height)
            .unwrap_or(0);

        let height = parent_height + 1;
        self.nodes.insert(
            id,
            EditorNode {
                parent,
                children,
                height,
                node,
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
    fn closest_common_parent(&self, node_a: &usize, _: &usize) -> Option<usize> {
        Some(self.parent_of(node_a).unwrap_or(*node_a))
    }
}
