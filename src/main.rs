use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut times = use_signal(|| 1);

    let exclamations = "!".repeat((*times)());

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(0, 109, 119)",
            direction: "vertical",
            main_align: "center",
            cross_align: "center",
            onclick: move |_| times += 1,
            label {
                width: "100%",
                font_size: "50",
                text_align: "center",
                color: "white",
                "Hello, World{exclamations}"
            }
        }
    )
}
