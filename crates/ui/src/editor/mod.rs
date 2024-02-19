pub mod document;
pub mod gutter;

use freya::{common::EventMessage, prelude::*};
use skia_safe::{
    font_style::{Slant, Weight, Width},
    Color, Font, FontMgr, FontStyle, Paint, Typeface,
};
use synk_core::document::Document;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};

#[allow(non_snake_case)]
#[component]
pub fn Editor(
    colors: Colors,
    line_height: f32,
    font_family: &'static str,
    font_size: f32,
) -> Element {
    let document = Document::new(
        "fn main() {\n    let x = \"Hello!\\n\";\n    println!(\"{x}\");\n}".to_string(),
    );

    let platform = use_platform();
    use_effect(move || {
        platform.send(EventMessage::RequestRerender).unwrap();
    });

    let canvas = use_canvas(
        &(document, font_size, font_family, line_height),
        |document, font_size, font_family, line_height| {
            Box::new(move |canvas, _, region| {
                let rope = document.rope.clone();
                canvas.translate((region.min_x(), region.min_y()));

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_color(Color::WHITE);

                let font_style = FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright);
                let font_family = FontMgr::new()
                    .match_family_style(font_family, font_style)
                    .unwrap();
                let font = Font::from_typeface(font_family, font_size);

                canvas.restore();
            })
        },
    );

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter { rope: document.rope.clone(), colors: colors.line_numbers, line_height }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            rect {
                width: "calc(100% - 50)",
                height: "100%",
                direction: "vertical",
                overflow: "clip",
                canvas_reference: canvas.attribute()
            }
        }
    }
}
