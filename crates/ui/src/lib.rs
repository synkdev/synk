pub mod theme;

use eframe::{App, NativeOptions};
use egui::ViewportBuilder;

pub struct Ui {
    text: String,
}

pub fn native_options(title: &'static str, app_id: &'static str) -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_title(title)
            .with_app_id(app_id)
            .with_active(true)
            .with_visible(true)
            .with_decorations(false),
        ..Default::default()
    }
}

pub fn launch(native_options: NativeOptions) -> anyhow::Result<()> {
    let _ = eframe::run_native(
        native_options.clone().viewport.title.unwrap().as_str(),
        native_options,
        Box::new(|cc| Box::new(Ui::new(cc))),
    );
    Ok(())
}

impl Ui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let catppuccin_theme = theme::Theme::new();
        cc.egui_ctx.set_visuals(catppuccin_theme.visuals());
        Ui {
            text: "some text".to_string(),
        }
    }
}

impl App for Ui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.label(self.text.as_str()));
    }
}
