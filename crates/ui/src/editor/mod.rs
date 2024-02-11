pub mod gutter;

use freya::prelude::*;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use crop::Rope;
use tree_sitter::{Language, Parser};

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors, contents: Rope) -> Element {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_rust::language())
        .expect("Error loading Rust grammar");
    let tree = parser.parse(contents.to_string(), None).unwrap();
    let mut cursor = tree.root_node().walk();

    for node in tree.root_node().children(&mut cursor) {
        println!("{}", node.kind());
    }

    rsx! {
        rect {
            background: "{colors.editor.background}",
            width: "100%",
            height: "calc(100% - 84)",
            direction: "horizontal",
            Gutter { rope: contents.clone(), colors: colors.line_numbers }
            VerticalSeparator { interactive: false, reverse: false, colors: colors.separator }
            rect { width: "calc(100% - 50)", height: "100%", direction: "vertical",
                for (_ , line) in contents.lines().enumerate() {
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
