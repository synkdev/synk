use freya::prelude::*;
use winit::{dpi::LogicalSize, window::WindowBuilder};

#[allow(dead_code)]
const APP_ID: &'static str = "synk";

fn main() {
    let builder_hook: WindowBuilderHook = Box::new(|builder| {
        #[allow(unused_mut)]
        let mut window_builder = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(1200, 700))
            .with_active(true)
            .with_visible(true)
            .with_resizable(true)
            .with_title("Synk Editor");

        #[allow(dead_code)]
        #[cfg(feature = "wayland")]
        {
            use winit::platform::wayland::WindowBuilderExtWayland as _;
            window_builder = window_builder.with_name(APP_ID, "");
        }

        #[allow(dead_code)]
        #[cfg(feature = "x11")]
        {
            use winit::platform::x11::WindowBuilderExtX11 as _;
            window_builder = window_builder.with_name(APP_ID, "");
        }

        *builder = window_builder;
    });

    launch_cfg(
        ui::SynkUI,
        LaunchConfig::<()>::builder()
            .with_window_builder(builder_hook)
            .build(),
    );
}
