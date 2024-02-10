use freya::prelude::*;

use crate::colors::EditorColors;
use crop::Rope;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: EditorColors, contents: Rope) -> Element {
    rsx! {
        rect { background: "{colors.background}", width: "100%", height: "calc(100% - 84)",
            for (_ , line) in contents.lines().enumerate() {
                rect {
                    background: "rgb(49, 50, 68)",
                    width: "100%",
                    height: "40",
                    direction: "horizontal",
                    cross_align: "center",
                    label { font_family: "JetBrains Mono", font_size: "16", color: "rgb(205, 214, 244)", "{line}" }
                }
            }
        }
    }
}
