use freya::prelude::*;
use winit::dpi::LogicalSize;

#[allow(dead_code)]
const APP_ID: &'static str = "synk";
const JETBRAINS_MONO: &[u8] = include_bytes!("../resources/fonts/jetbrains_mono.ttf");

fn main() {
    let builder_hook: WindowBuilderHook = Box::new(|mut builder| {
        builder = builder
            .with_inner_size(LogicalSize::new(1400, 800))
            .with_active(true)
            .with_visible(true)
            .with_resizable(true)
            .with_title("Synk Editor");

        #[allow(dead_code)]
        #[cfg(feature = "wayland")]
        {
            use winit::platform::wayland::WindowBuilderExtWayland as _;
            builder = builder.with_name(APP_ID, "");
        }

        #[allow(dead_code)]
        #[cfg(feature = "x11")]
        {
            use winit::platform::x11::WindowBuilderExtX11 as _;
            builder = builder.with_name(APP_ID, "");
        }
        builder
    });

    launch_cfg(
        ui::SynkUI,
        LaunchConfig::<()>::builder()
            .with_font("JetBrains Mono", JETBRAINS_MONO)
            .with_window_builder(builder_hook)
            .build(),
    );
}
