#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() -> anyhow::Result<()> {
    dioxus_desktop::launch(App);
    Ok(())
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
