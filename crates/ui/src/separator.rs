use freya::prelude::*;

use crate::colors::SeparatorColors;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator(colors: SeparatorColors) -> Element {
    let mut width = use_signal(|| 2);
    // let active = colors.active.as_str();
    // let default = colors.default.as_str();
    let mut background = use_signal(|| "{colors.default}");

    rsx! {
        rect {
            height: "100%",
            width: "{width.read()}",
            background: "{background}",
            onmouseover: move |_| {
                background.set("{colors.active}");
                width.set(6);
            },
            onmouseleave: move |_| {
                background.set("{colors.default}");
                width.set(2);
            },
            label { "{background.read()}" }
        }
    }
}
