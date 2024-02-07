use freya::prelude::*;

fn main() {
    launch_cfg(
        ui::SynkUI,
        LaunchConfig::<()>::builder()
            .with_height(700.0)
            .with_width(1200.0)
            .with_title("Synk Editor")
            .build(),
    );
}
