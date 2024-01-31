pub mod theme;

use eframe::{App, NativeOptions};
use egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder};

pub struct Ui {
    text: String,
}

pub fn native_options(title: &'static str, app_id: &'static str) -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
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
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "jetbrains_mono".to_owned(),
            FontData::from_static(include_bytes!(
                "../../../resources/fonts/JetBrainsMono-Light.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "jetbrains_mono".to_owned());

        cc.egui_ctx.set_visuals(catppuccin_theme.visuals());
        cc.egui_ctx.set_fonts(fonts);
        Ui {
            text: "some text".to_string(),
        }
    }
}

impl App for Ui {
    fn update(&mut self, cx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .width_range(150.0..=400.0)
            .default_width(250.0)
            .show_animated(cx, true, |ui| {
                ui.horizontal(|ui| ui.label("files"));
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            });
        egui::CentralPanel::default().show(cx, |ui| ui.label(self.text.as_str()));
    }
}
