pub mod colors;

use colors::Colors;
use freya::prelude::*;

#[allow(non_snake_case)]
pub fn SynkUI() -> Element {
    let colors = Colors::new();
    println!("{0}", colors.background);
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            background: "{colors.background}",
            color: "{colors.foreground",
            main_align: "center",
            cross_align: "center",
            label { "hello" }
        }
    }
}
