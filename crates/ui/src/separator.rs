use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator() -> Element {
    let mut width = use_signal(|| 2);
    let mut background = use_signal(|| "rgb(69, 71, 90)");

    rsx! {
        rect {
            height: "100%",
            width: "{width.read()}",
            background: "{background.read()}",
            onmouseover: move |_| {
                background.set("rgb(147, 153, 178)");
                width.set(6);
            },
            onmouseleave: move |_| {
                background.set("rgb(69, 71, 90)");
                width.set(2);
            }
        }
    }
}
