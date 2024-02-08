pub mod colors;
pub mod editor;
pub mod separator;
pub mod sidebar;
pub mod statusbar;
pub mod tab_bar;

use colors::Colors;
use freya::prelude::*;

use crate::{
    separator::{HorizontalSeparator, VerticalSeparator},
    sidebar::Sidebar,
    statusbar::Statusbar,
};

#[allow(non_snake_case)]
pub fn SynkUI() -> Element {
    let colors = Colors::new();
    let sidebar_width = use_signal(|| 300_usize);

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            background: "{colors.background}",
            color: "{colors.foreground}",
            cross_align: "center",
            font_family: "JetBrains Mono",
            overflow: "clip",
            Sidebar { width: sidebar_width, colors: colors.sidebar }
            VerticalSeparator {}
            rect { width: "100%", height: "100%", direction: "vertical",
                HorizontalSeparator {}
                Statusbar { colors: colors.statusbar }
            }
        }
    }
}
