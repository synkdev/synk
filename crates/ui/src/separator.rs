use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn VerticalSeparator() -> Element {
    let hover_anim = use_animation_transition(TransitionAnimation::SineInOut(150), (), |_| {
        vec![
            Transition::new_color("rgb(69, 71, 90)", "rgb(243, 139, 168)"),
            Transition::new_size(2.0, 6.0),
        ]
    });

    let color = hover_anim.get(0).unwrap().as_color();
    let width = hover_anim.get(1).unwrap().as_size();

    rsx! {
        rect {
            height: "100%",
            width: "{width}",
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
}
