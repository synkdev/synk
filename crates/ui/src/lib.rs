pub mod colors;

use colors::Colors;
use freya::prelude::*;

pub struct SynkUI {
    pub colors: Colors,
}

impl SynkUI {
    pub fn new() -> SynkUI {
        SynkUI {
            colors: Colors::new(),
        }
    }
    pub fn render(&self) -> Element {
        let color = "rgb(10, 20, 10)".to_string();
        rsx! {
            rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                background: "{color}",
                main_align: "center",
                cross_align: "center",
                label {
                    "hello"
                }
            }
        }
    }
}
