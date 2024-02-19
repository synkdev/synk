pub mod document;
pub mod gutter;

use freya::torin::*;
use freya::{common::EventMessage, prelude::*};
use skia_safe::TextBlob;
use skia_safe::{
    font_style::{Slant, Weight, Width},
    Color, Font, FontMgr, FontStyle, Paint,
};
use synk_core::document::Document;

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
        platform.send(EventMessage::RequestRerender).unwrap();
    });

    let canvas = use_canvas(&config, |config| {
        Box::new(move |canvas, _, region| {
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
            let mut next_line_start = region.min_y();

            for (line_idx, line) in rope.lines().enumerate() {
                let torin_rect = Node::from_size_and_direction(
                    Size::Pixels(Length::new(region.width())),
                    Size::Pixels(Length::new(config.line_height)),
                    DirectionMode::Horizontal,
                );
                let mut next_char_start = region.min_x();
                for (byte_idx, char) in line.chars().enumerate() {
                    let text_blob = TextBlob::from_str(char.to_string().as_str(), &font);
                    let text_bounds = font.measure_text(char.to_string().as_str(), Some(&paint)).1;
                    println!("{}: {}", byte_idx, text_bounds.width());
                }
                next_line_start += config.line_height;
            }

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
