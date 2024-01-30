use ui::Ui;

fn main() -> anyhow::Result<()> {
    let ui = Ui::new();
    let native_options = ui.native_options();
    ui.launch(native_options)?;
}
