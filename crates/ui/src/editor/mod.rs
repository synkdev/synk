pub mod document;
pub mod gutter;

use freya::prelude::*;
use synk_core::document::Document;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use document::Document as EditorDocument;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors) -> Element {
    let document = Document::new(
        "fn main() {\n    let x = \"Hello!\\n\";\n    println!(\"{x}\");\n}".to_string(),
    );
    let canvas = use_canvas();

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter { rope: document.rope.clone(), colors: colors.line_numbers }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            EditorDocument { rope: document.rope.clone(), colors: colors.editor.clone() }
        }
    }
}
