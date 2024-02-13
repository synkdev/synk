pub mod gutter;

use freya::prelude::*;
use synk_core::{document::Document, highlighter::TSParser};

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use ropey::Rope;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors) -> Element {
    let document = Document::new("fn main() {\n    println!(\"Hello World!\");\n}".to_string());
    let mut parser = document.ts_parser;
    let scope = parser.get_scope(0);

    println!("{scope:?}");

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter { rope: document.rope.clone(), colors: colors.line_numbers }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            rect { width: "calc(100% - 50)", height: "100%", direction: "vertical",
                for (_ , line) in document.rope.lines().enumerate() {
                    rect {
                        background: "{colors.editor.background}",
                        width: "100%",
                        height: "40",
                        direction: "horizontal",
                        cross_align: "center",
                        label {
                            font_family: "JetBrains Mono",
                            font_size: "16",
                            color: "{colors.editor.foreground}",
                            "{line}"
                        }
                    }
                }
            }
        }
    }
}
