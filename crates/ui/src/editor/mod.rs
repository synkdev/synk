pub mod gutter;

use freya::prelude::*;
use synk_core::{
    document::Document,
    highlighter::{languages::Languages, theme::Theme, RopeProvider, TSParser},
};
use tree_sitter::QueryCursor;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use ropey::Rope;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors) -> Element {
    let document = Document::new(
        r#"fn main() {
    println!("Hello World!");
}"#
        .to_string(),
    );
    let parser = TSParser::new(Languages::Rust, document.rope.clone());
    let query = parser.query;
    let rope = parser.rope;
    let tree = parser.tree;
    let mut query_cursor = QueryCursor::new();
    query_cursor.set_byte_range(rope.line_to_byte(0)..rope.line_to_byte(rope.len_lines()));
    let theme = Theme::default();

    let mut matches = query_cursor
        .matches(&query, tree.root_node(), RopeProvider(rope.slice(..)))
        .peekable();

    let line_idx = 0;

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter { rope: document.rope.clone(), colors: colors.line_numbers }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            rect { width: "calc(100% - 50)", height: "100%", direction: "vertical",
                for (line_idx , line) in document.rope.lines().enumerate() {
                    {
                        let char_idx = rope.line_to_char(line_idx);
                        let line_start_byte = rope.char_to_byte(char_idx);
                        rsx! {
                            rect {
                                background: "{colors.editor.background}",
                                width: "100%",
                                height: "40",
                                direction: "horizontal",
                                cross_align: "center",
                                paragraph { width: "100%", max_lines: "1", font_size: "16", font_family: "JetBrains Mono",
                                    for (byte_idx , char) in line.chars().enumerate() {
                                        {
                                            println!("{char}:{byte_idx}");
                                            let scope = TSParser::get_scope(&query, &mut matches, line_start_byte + byte_idx).unwrap_or("".to_string());
                                            let color = theme.get_color_from_scope(scope);
                                            rsx!(
                                                text {
                                                    color: "{color.color}",
                                                    font_weight: "{color.weight}",
                                                    "{char}"
                                                }
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
