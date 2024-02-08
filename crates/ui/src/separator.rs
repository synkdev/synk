use freya::prelude::*;

use crate::colors::SeparatorColors;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator(colors: SeparatorColors) -> Element {
    let mut width = use_signal(|| 2);
    let mut background = use_signal(|| "{colors.default}");
    rsx! {
        rect {
            height: "100%",
            width: "width.read()",
            background: "background.read()",
            onmouseover: move |_| {
                background.set("{colors.active}");
                width.set(4);
            },
            onmouseleave: move |_| {
                background.set("{colors.default}");
                width.set(2);
            }
        }
    }
}
