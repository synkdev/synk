use crate::colors::TabBarColors;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn TabBar(colors: TabBarColors) -> Element {
    rsx! {rect { width: "100%", height: "60", background: "{colors.bar_bg}", color: "{colors.tab_fg}" }}
}
