use freya::prelude::*;
use winit::window::CursorIcon;

use crate::colors::SeparatorColors;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator(
    colors: SeparatorColors,
    interactive: bool,
    reverse: bool,
    mut callback: Option<Signal<isize>>,
) -> Element {
    if interactive {
        let hover_anim = use_animation(move |cx| {
            (
                cx.with(
                    AnimColor::new(colors.default.as_str(), colors.active.as_str())
                        .time(150)
                        .ease(Ease::InOut),
                ),
                cx.with(AnimNum::new(1.0, 6.0).time(150).ease(Ease::InOut)),
            )
        });

        let platform = use_platform();

        let anim = hover_anim.read();
        let (color, width) = anim.get();

        let mut position = use_signal(|| 0usize);
        let mut is_clicked = use_signal::<Option<CursorPoint>>(|| None);

        let onglobalmouseover = move |e: MouseEvent| {
            if let Some(clicked) = *is_clicked.read() {
                if let Some(mut callback) = callback {
                    let extend_size = (e.get_screen_coordinates().x - clicked.x) as isize;
                    *callback.write() = extend_size;
                    position.set(e.get_screen_coordinates().x as usize);
                }
            }
        };

        let onmousedown = move |e: MouseEvent| {
            is_clicked.set(Some(e.get_element_coordinates()));
            platform.set_cursor(CursorIcon::ColResize)
        };

        let onglobalclick = move |_: MouseEvent| {
            platform.set_cursor(CursorIcon::Default);
            is_clicked.set(None);
        };

        rsx! {
            rect {
                width: "12",
                height: "100%",
                onglobalmouseover,
                onglobalclick,
                onmousedown,
                onmouseover: move |_| {hover_anim.read().start(); },
                onmouseleave: move |_| {hover_anim.read().reverse();},
                direction: "horizontal",
                if reverse {
                    rect { height: "100%", width: "{width.read().as_f32()}", background: "{color.read().as_string()}" }
                    rect { height: "100%", width: "calc(100% - {width.read().as_f32()})" }
                } else {
                    rect { height: "100%", width: "{width.read().as_f32()}", background: "{color.read().as_string()}" }
                    rect { height: "100%", width: "{width.read().as_f32()}" }
                }

            }
        }
    } else {
        rsx! { rect { height: "100%", width: "1", background: "{colors.default}" } }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn HorizontalSeparator(
    colors: SeparatorColors,
    interactive: bool,
    reverse: bool,
    mut callback: Option<Signal<isize>>,
) -> Element {
    if interactive {
        let hover_anim = use_animation(move |cx| {
            (
                cx.with(
                    AnimColor::new(colors.default.as_str(), colors.active.as_str())
                        .time(150)
                        .ease(Ease::InOut),
                ),
                cx.with(AnimNum::new(1.0, 6.0).time(150).ease(Ease::InOut)),
            )
        });

        let anim = hover_anim.read();
        let (color, height) = anim.get();

        let mut position = use_signal(|| 0usize);
        let mut is_clicked = use_signal::<Option<CursorPoint>>(|| None);

        let onglobalmouseover = move |e: MouseEvent| {
            if let Some(clicked) = *is_clicked.read() {
                if let Some(mut callback) = callback {
                    let extend_size = (e.get_screen_coordinates().x - clicked.x) as isize;
                    *callback.write() = extend_size;
                    position.set(e.get_screen_coordinates().x as usize);
                }
            }
        };

        let onmousedown = move |e: MouseEvent| {
            is_clicked.set(Some(e.get_element_coordinates()));
        };

        let onglobalclick = move |_: MouseEvent| {
            is_clicked.set(None);
        };

        rsx! {
            rect {
                height: "12",
                width: "100%",
                onglobalmouseover,
                onglobalclick,
                onmousedown,
                onmouseover: move |_| {hover_anim.read().start(); },
                onmouseleave: move |_| {hover_anim.read().reverse();},
                direction: "horizontal",
                if reverse {
                    rect { width: "100%", height: "{height.read().as_f32()}", background: "{color.read().as_string()}" }
                    rect { width: "100%", height: "calc(100% - {height.read().as_f32()})" }
                } else {
                    rect { width: "100%", height: "calc(100% - {height.read().as_f32()})" }
                    rect { width: "100%", height: "{height.read().as_f32()}", background: "{color.read().as_string()}" }
                }

            }
        }
    } else {
        rsx! { rect { width: "100%", height: "1", background: "{colors.default}" } }
    }
}
