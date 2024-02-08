use crate::colors::StatusbarColors;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Statusbar(colors: StatusbarColors) -> Element {
    rsx! { rect { width: "100%", height: "40", background: "{colors.background}" } }
}
