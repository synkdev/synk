pub mod gutter;

use freya::prelude::*;
use synk_core::{
    document::Document,
    highlighter::{languages::Languages, RopeProvider, TSParser},
};
use tree_sitter::QueryCursor;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use ropey::Rope;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors) -> Element {
    let document = Document::new("fn main() {\n    println!(\"Hello World!\");\n}".to_string());
    let parser = TSParser::new(Languages::Rust, document.rope.clone());
    let query = parser.query;
    let rope = parser.rope;
    let tree = parser.tree;
    let mut query_cursor = QueryCursor::new();
    query_cursor.set_byte_range(rope.line_to_byte(0)..rope.line_to_byte(rope.len_lines()));

    let matches = query_cursor
        .matches(&query, tree.root_node(), RopeProvider(rope.slice(..)))
        .peekable();

    let scope = TSParser::get_scope(&query, matches, 0);

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
