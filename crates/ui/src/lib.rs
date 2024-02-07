pub mod colors;

use freya::prelude::*;

pub struct SynkUI;

impl SynkUI {
    pub fn render() -> Element {
        let color = "rgb(10, 20, 10)";
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
