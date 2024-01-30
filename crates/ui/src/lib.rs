use eframe::NativeOptions;
use egui::ViewportBuilder;

pub struct Ui {
    title: &'static str,
    app_id: &'static str,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            title: "Synk Editor",
            app_id: "synk",
        }
    }

    pub fn native_options(&self) -> NativeOptions {
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([1200, 700])
                .with_title(self.title)
                .with_app_id(self.app_id)
                .with_active(true)
                .with_visible(true)
                .with_decorations(false),
            ..Default::default()
        }
    }
    pub fn launch(&self, native_options: NativeOptions) -> anyhow::Result<()> {
        eframe::run_native(self.title, native_options, None)
    }
}
