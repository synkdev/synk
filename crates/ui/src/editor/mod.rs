pub mod document;
pub mod dom;
pub mod gutter;

use freya::torin::*;
use freya::{common::EventMessage, prelude::*};
use skia_safe::TextBlob;
use skia_safe::{
    font_style::{Slant, Weight, Width},
    Color, Font, FontMgr, FontStyle, Paint,
};
use synk_core::document::Document;

use crate::editor::dom::{EditorDom, TextMeasurer};
use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};

#[derive(Clone, PartialEq)]
pub struct EditorConfig {
    pub document: Document,
    pub line_height: f32,
    pub font_family: &'static str,
    pub font_size: f32,
}

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors, config: EditorConfig) -> Element {
    let platform = use_platform();
    use_effect(move || {
        platform
            .send(EventMessage::RequestRerender)
            .expect("Couldn't request rerender");
    });

    let canvas = use_canvas(&config, |config| {
        Box::new(move |canvas, _, region| {
            let mut was_measured = false;
            let mut was_drawn = false;
            let rope = config.document.rope.clone();
            canvas.translate((region.min_x(), region.min_y()));

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(Color::WHITE);

            let font_style = FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright);
            let font_family = FontMgr::new()
                .match_family_style(config.font_family, font_style)
                .unwrap();
            let font = Font::from_typeface(font_family, config.font_size);

            // Dom stuff
            let mut torin = Torin::<usize>::new();
            let mut dom = EditorDom::default();
            let mut measurer = Some(TextMeasurer {
                font: &font,
                paint: &paint,
                dom: dom.clone(),
            });

            // let mut next_line_start = region.min_y();

            if !was_measured {
                println!("measuring");
                // Add root node for the editor
                dom.add(
                    0,
                    vec![1],
                    Node::from_size_and_alignments_and_direction(
                        Size::Percentage(Length::new(100.0)),
                        Size::Percentage(Length::new(100.0)),
                        Alignment::Center,
                        Alignment::Start,
                        DirectionMode::Vertical,
                    ),
                    None,
                    dom::NodeType::Root,
                );

                for (line_idx, line) in rope.lines().enumerate() {
                    println!("adding line");
                    dom.add(
                        line_idx,
                        vec![2],
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(region.width())),
                            Size::Pixels(Length::new(config.line_height)),
                            DirectionMode::Horizontal,
                        ),
                        Some(0),
                        dom::NodeType::Line {
                            chars: line.chars().map(|c| c).collect(),
                        },
                    );
                    for (byte_idx, char) in line.chars().enumerate() {
                        println!("adding char");
                        dom.add(
                            byte_idx,
                            vec![],
                            Node::default(),
                            Some(line_idx),
                            dom::NodeType::Char(char),
                        );
                        // let text_blob = TextBlob::from_str(char.to_string().as_str(), &font);
                        // let text_bounds = font.measure_text(char.to_string().as_str(), Some(&paint)).1;
                    }
                    // next_line_start += config.line_height;
                }
                torin.measure(
                    0,
                    Rect::new(region.min(), region.size),
                    &mut measurer,
                    &mut dom,
                );
                was_measured = true;
                for (id, node) in &torin.results {
                    println!("{id:?} -> {:?}", node.area);
                }
            }

            if !was_drawn {}

            canvas.restore();
        })
    });

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter {
                rope: config.document.rope.clone(),
                colors: colors.line_numbers,
                line_height: config.line_height
            }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            rect {
                width: "calc(100% - 50)",
                height: "100%",
                direction: "vertical",
                overflow: "clip",
                onclick: move |_| { println!("clicked") },
                canvas_reference: canvas.attribute()
            }
        }
    }
}
