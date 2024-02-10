pub mod gutter;

use freya::prelude::*;

use crate::{colors::Colors, editor::gutter::Gutter, separator::VerticalSeparator};
use crop::Rope;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: Colors, contents: Rope) -> Element {
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
                        background: "rgb(49, 50, 68)",
                        width: "100%",
                        height: "40",
                        direction: "horizontal",
                        cross_align: "center",
                        label {
                            font_family: "JetBrains Mono",
                            font_size: "16",
                            color: "rgb(205, 214, 244)",
                            "{line}"
                        }
                    }
                }
            }
        }
    }
}
