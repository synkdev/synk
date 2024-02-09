use freya::{dioxus::html::del, prelude::*};

use crate::colors::SeparatorColors;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator(
    colors: SeparatorColors,
    interactive: bool,
    mut callback: Option<Signal<usize>>,
) -> Element {
    if interactive {
        let hover_anim =
            use_animation_transition(TransitionAnimation::SineInOut(150), (), move |_| {
                vec![
                    Transition::new_color(colors.default.as_str(), colors.active.as_str()),
                    Transition::new_size(2.0, 6.0),
                ]
            });

        let color = hover_anim.get(0).unwrap().as_color();
        let width = hover_anim.get(1).unwrap().as_size();

        let mut is_hovered = use_signal(|| false);
        let mut position = use_signal(|| 0usize);
        let mut is_clicked = use_signal(|| false);

        let onmouseleave = {
            to_owned![hover_anim];
            move |_: MouseEvent| {
                hover_anim.reverse();
                is_hovered.set(false);
                is_clicked.set(false);
            }
        };

        let onmouseover = {
            to_owned![hover_anim];
            move |e: MouseEvent| {
                hover_anim.start();
                is_hovered.set(true);
                if *is_clicked.read() {
                    let x = e.get_screen_coordinates().x as usize;
                    if let Some(mut callback) = callback {
                        let extend_size = x.saturating_sub(*position.read());
                        if extend_size == 0 {
                            *callback.write() -= 1;
                        } else {
                            *callback.write() += 1;
                        }
                        position.set(e.get_screen_coordinates().x as usize);
                    }
                }
            }
        };

        let onmousedown = move |e: MouseEvent| {
            position.set(e.get_screen_coordinates().x as usize);
            is_clicked.set(true);
        };

        let onclick = move |_: MouseEvent| {
            is_clicked.set(false);
        };

        rsx! {
            rect {
                width: "10",
                height: "100%",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                onclick: onclick,
                onmousedown: onmousedown,
                direction: "horizontal",
                rect { height: "100%", width: "calc(100% - {width})" }
                rect { height: "100%", width: "{width}", background: "{color}" }
            }
        }
    } else {
        rsx! { rect { height: "100%", width: "2", background: "{colors.default}" } }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn HorizontalSeparator(
    colors: SeparatorColors,
    interactive: bool,
    mut callback: Option<Signal<usize>>,
) -> Element {
    if interactive {
        let hover_anim = use_animation_transition(TransitionAnimation::SineInOut(150), (), |_| {
            vec![
                Transition::new_color("rgb(69, 71, 90)", "rgb(243, 139, 168)"),
                Transition::new_size(2.0, 6.0),
            ]
        });

        let color = hover_anim.get(0).unwrap().as_color();
        let height = hover_anim.get(1).unwrap().as_size();

        let mut is_hovered = use_signal(|| false);
        let mut position = use_signal(|| 0usize);
        let mut is_clicked = use_signal(|| false);

        let onmouseleave = {
            to_owned![hover_anim];
            move |_: MouseEvent| {
                hover_anim.reverse();
                is_hovered.set(false);
                is_clicked.set(false);
            }
        };

        let onmouseover = {
            to_owned![hover_anim];
            move |e: MouseEvent| {
                hover_anim.start();
                is_hovered.set(true);
                if *is_clicked.read() {
                    let x = e.get_screen_coordinates().x as usize;
                    if let Some(mut callback) = callback {
                        let extend_size = x.saturating_sub(*position.read());
                        if extend_size == 0 {
                            *callback.write() -= 1;
                        } else {
                            *callback.write() += 1;
                        }
                        position.set(e.get_screen_coordinates().x as usize);
                    }
                }
            }
        };

        let onmousedown = move |e: MouseEvent| {
            position.set(e.get_screen_coordinates().x as usize);
            is_clicked.set(true);
        };

        let onclick = move |_: MouseEvent| {
            is_clicked.set(false);
        };

        rsx! {
            rect {
                width: "10",
                height: "100%",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                onclick: onclick,
                onmousedown: onmousedown,
                direction: "vertical",
                rect { width: "100%", height: "calc(100% - {height})" }
                rect { width: "100%", height: "{height}", background: "{color}" }
            }
        }
    } else {
        rsx! { rect { width: "100%", height: "2", background: "{colors.default}" } }
    }
}
