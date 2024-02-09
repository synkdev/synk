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
                    Transition::new_size(6.0, 6.0),
                ]
            });

        let color = hover_anim.get(0).unwrap().as_color();
        let width = hover_anim.get(1).unwrap().as_size();

        let mut hovering = use_signal(|| false);
        let mut position = use_signal(|| 0usize);
        let mut clicking = use_signal(|| false);

        let onmouseleave = {
            to_owned![hover_anim];
            move |_: MouseEvent| {
                hover_anim.reverse();
                hovering.set(false);
                clicking.set(false);
            }
        };

        let onmouseover = {
            to_owned![hover_anim];
            move |e: MouseEvent| {
                hover_anim.start();
                hovering.set(true);
                if *clicking.read() {
                    let x = e.get_screen_coordinates().x as usize;
                    println!("{}", clicking.read());
                    if let Some(mut callback) = callback {
                        let extend_size = x.saturating_sub(*position.read());
                        println!("{extend_size}");
                        *callback.write() += extend_size;
                        position.set(e.get_screen_coordinates().x as usize);
                    }
                }
            }
        };

        let onmousedown = move |e: MouseEvent| {
            position.set(e.get_screen_coordinates().x as usize);
            clicking.set(true);
        };

        let onclick = move |_: MouseEvent| {
            clicking.set(false);
        };

        rsx! {
            rect {
                height: "100%",
                width: "{width}",
                background: "{color}",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                onclick: onclick,
                onmousedown: onmousedown
            }
        }
    } else {
        rsx! { rect { height: "100%", width: "2", background: "{colors.default}" } }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn HorizontalSeparator(colors: SeparatorColors, interactive: bool) -> Element {
    if interactive {
        let hover_anim = use_animation_transition(TransitionAnimation::SineInOut(150), (), |_| {
            vec![
                Transition::new_color("rgb(69, 71, 90)", "rgb(243, 139, 168)"),
                Transition::new_size(2.0, 6.0),
            ]
        });

        let color = hover_anim.get(0).unwrap().as_color();
        let height = hover_anim.get(1).unwrap().as_size();

        rsx! {
            rect {
                width: "100%",
                height: "{height}",
                background: "{color}",
                onmouseover: {
                    to_owned![hover_anim];
                    move |_| { hover_anim.start() }
                },
                onmouseleave: {
                    to_owned![hover_anim];
                    move |_| { hover_anim.reverse() }
                }
            }
        }
    } else {
        rsx! { rect { width: "100%", height: "2", background: "{colors.default}" } }
    }
}
