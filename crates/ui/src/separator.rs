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
        let mut hover_anim =
            use_animation_transition(TransitionAnimation::SineInOut(150), (), move |_| {
                vec![
                    Transition::new_color(colors.default.as_str(), colors.active.as_str()),
                    Transition::new_size(1.0, 6.0),
                ]
            });

        let platform = use_platform();

        let color = hover_anim.get(0).unwrap().as_color();
        let width = hover_anim.get(1).unwrap().as_size();

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
                width: "12",
                height: "100%",
                onglobalmouseover,
                onglobalclick,
                onmousedown,
                onmouseover: move |_| {hover_anim.start(); platform.set_cursor(CursorIcon::ColResize)},
                onmouseleave: move |_| {hover_anim.reverse(); platform.set_cursor(CursorIcon::Default)},
                direction: "horizontal",
                if reverse {
                    rect { height: "100%", width: "{width}", background: "{color}" }
                    rect { height: "100%", width: "calc(100% - {width})" }
                } else {
                    rect { height: "100%", width: "calc(100% - {width})" }
                    rect { height: "100%", width: "{width}", background: "{color}" }
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
        let hover_anim =
            use_animation_transition(TransitionAnimation::SineInOut(150), (), move |_| {
                vec![
                    Transition::new_color(colors.default.as_str(), colors.active.as_str()),
                    Transition::new_size(1.0, 6.0),
                ]
            });

        let color = hover_anim.get(0).unwrap().as_color();
        let height = hover_anim.get(1).unwrap().as_size();

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
                onmouseover: {to_owned![hover_anim]; move |_| {hover_anim.start()}},
                onmouseleave: {to_owned![hover_anim]; move |_| {hover_anim.reverse()}},
                direction: "horizontal",
                if reverse {
                    rect { width: "100%", height: "{height}", background: "{color}" }
                    rect { width: "100%", height: "calc(100% - {height})" }
                } else {
                    rect { width: "100%", height: "calc(100% - {height})" }
                    rect { width: "100%", height: "{height}", background: "{color}" }
                }

            }
        }
    } else {
        rsx! { rect { width: "100%", height: "1", background: "{colors.default}" } }
    }
}
