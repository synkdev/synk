pub mod colors;
pub mod separator;
pub mod sidebar;

use colors::Colors;
use freya::prelude::*;

use crate::{separator::VerticalSeparator, sidebar::Sidebar};

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
            VerticalSeparator { colors: colors.separator }
            label {
                font_weight: "bold",
                font_style: "italic",
                onclick: move |_| *sidebar_width.write() += 20,
                "hello"
            }
        }
    }
}
