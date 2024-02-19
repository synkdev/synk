use freya::prelude::*;
use ropey::Rope;

use crate::colors::LineNumberColors;

#[allow(non_snake_case)]
#[component]
pub fn Gutter(rope: Rope, colors: LineNumberColors, line_height: f32) -> Element {
    rsx! {
        rect { height: "100%", width: "50", direction: "vertical", background: "{colors.line_numbers_bg}",
            for (line_number , _) in rope.lines().enumerate() {
                rect { width: "100%", height: "40", cross_align: "center", main_align: "center",
                    label {
                        font_family: "JetBrains Mono",
                        font_size: "16",
                        font_weight: "bold",
                        color: "{colors.line_numbers_fg}",
                        "{line_number + 1}"
                    }
                }
            }
        }
    }
}
