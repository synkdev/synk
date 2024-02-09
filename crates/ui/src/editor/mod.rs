use freya::prelude::*;

use crate::colors::EditorColors;

#[allow(non_snake_case)]
#[component]
pub fn Editor(colors: EditorColors) -> Element {
    rsx! {rect { background: "{colors.background}", width: "100%", height: "calc(100% - 84)" }}
}
