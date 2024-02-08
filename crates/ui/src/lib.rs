pub mod colors;
pub mod sidebar;

use std::ops::Add;

use colors::Colors;
use freya::prelude::*;

use crate::sidebar::Sidebar;

#[allow(non_snake_case)]
pub fn SynkUI() -> Element {
    let colors = Colors::new();
    let mut sidebar_width = use_signal(|| 300_usize);

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            background: "{colors.background}",
            color: "{colors.foreground}",
            cross_align: "center",
            font_family: "JetBrains Mono",
            Sidebar { width: sidebar_width, colors: colors.sidebar }
            label {
                font_weight: "bold",
                font_style: "italic",
                onclick: move |_| *sidebar_width.write() += 20,
                "hello"
            }
        }
    }
}
