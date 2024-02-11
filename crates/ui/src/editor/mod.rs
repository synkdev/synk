pub mod gutter;

use freya::prelude::*;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use crop::Rope;
use tree_sitter::{Language, Parser};

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors, contents: Rope) -> Element {
    let mut text = "fn main() {\n    println!(\"Hello, world!\");\n}".as_bytes();
    let mut callback = |offset: usize, _: tree_sitter::Point| -> &[u8] {
        // Return a slice of UTF-8 encoded text starting at the given byte offset
        &text[offset..]
    };

    // Create a Tree-sitter parser and set the language (e.g., Rust)
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_rust::language())
        .expect("Error loading Rust grammar");

    // Parse the UTF-8 text provided by the callback
    let tree = parser.parse_with(&mut callback, None);

    // Handle the parsed tree
    match tree {
        Some(parsed_tree) => {
            // Use the parsed_tree
            println!(
                "Tree parsed successfully: {:?}",
                parsed_tree.root_node().to_sexp()
            );
        }
        None => {
            println!("Parsing failed.");
        }
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
