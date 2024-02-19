pub mod colors;
pub mod editor;
pub mod separator;
pub mod sidebar;
pub mod statusbar;
pub mod tab_bar;

use colors::Colors;
use freya::prelude::*;
use synk_core::document::Document;

use crate::{
    editor::{Editor, EditorConfig},
    separator::{HorizontalSeparator, VerticalSeparator},
    sidebar::Sidebar,
    statusbar::Statusbar,
    tab_bar::TabBar,
};

#[allow(non_snake_case)]
pub fn SynkUI() -> Element {
    let colors = Colors::new();
    let sidebar_width = use_signal(|| 300_isize);
    let document = Document::new(
        "fn main() {\n    let x = \"Hello!\\n\";\n    println!(\"{x}\");\n}".to_string(),
    );

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
                Sidebar { width: sidebar_width, colors: colors.clone().sidebar }
                VerticalSeparator {
                    colors: colors.separator.clone(),
                    interactive: true,
                    callback: sidebar_width,
                    reverse: false
                }
                rect { width: "calc(100% - {sidebar_width})", height: "100%", direction: "vertical",
                    TabBar { colors: colors.clone().tab_bar }
                    HorizontalSeparator { colors: colors.separator.clone(), interactive: false, reverse: false }
                    Editor {
                        colors: colors.clone(),
                        config: EditorConfig {
        line_height: 36.0,
        font_family: "JetBrains Mono",
        font_size: 16.0,
        document,
    }
                    }
                    HorizontalSeparator { colors: colors.separator.clone(), interactive: false, reverse: false }
                    Statusbar { colors: colors.statusbar }
                }
            }
        }
}
