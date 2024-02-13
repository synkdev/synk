pub mod gutter;

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use freya::prelude::*;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use crop::Rope;
use tree_sitter::{Language, Parser, Query, QueryCursor};

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors, contents: Rope) -> Element {
    let code = contents.to_string();
    let rust_lang = tree_sitter_rust::language();
    let highlights_file = read_to_string(PathBuf::from(
        "/home/mik3y/projects/repos/synk/resources/syntaxes/rust.scm",
    ))
    .unwrap();
    let mut parser = Parser::new();

    parser.set_language(rust_lang).unwrap();
    let highlights = Query::new(rust_lang, &highlights_file).unwrap();
    let mut query_cursor = QueryCursor::new();
    let tree = parser
        .parse_with(&mut |index, _| &code.as_bytes()[index..], None)
        .unwrap();

    println!("{tree:?}");

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
