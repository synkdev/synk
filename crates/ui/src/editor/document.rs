use freya::prelude::*;
use ropey::Rope;
use synk_core::highlighter::{languages::Languages, theme::SyntaxTheme, RopeProvider, TSParser};
use tree_sitter::QueryCursor;

use crate::colors::EditorColors;

#[allow(non_snake_case)]
#[component]
pub fn Document(rope: Rope, colors: EditorColors) -> Element {
    let parser = TSParser::new(Languages::Rust, rope);
    let query = parser.query;
    let rope = parser.rope;
    let tree = parser.tree;
    let mut query_cursor = QueryCursor::new();
    query_cursor.set_byte_range(rope.line_to_byte(0)..rope.line_to_byte(rope.len_lines()));
    let theme = SyntaxTheme::default();

    let mut matches = query_cursor
        .matches(&query, tree.root_node(), RopeProvider(rope.slice(..)))
        .peekable();
    rsx! {
        rect { width: "calc(100% - 50)", height: "100%", direction: "vertical",
            for (line_idx , line) in rope.lines().enumerate() {
                {
                    let char_idx = rope.line_to_char(line_idx);
                    let line_start_byte = rope.char_to_byte(char_idx);
                    rsx! {
                        rect {
                            background: "{colors.background}",
                            width: "100%",
                            height: "40",
                            direction: "horizontal",
                            cross_align: "center",
                            paragraph { width: "100%", max_lines: "1", font_size: "16", font_family: "JetBrains Mono",
                                for (byte_idx , char) in line.chars().enumerate() {
                                    {
                                        let scope = TSParser::get_scope(&query, &mut matches, line_start_byte + byte_idx).unwrap_or("".to_string());
                                        let color = theme.get_char_style(scope);
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
