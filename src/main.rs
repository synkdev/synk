fn main() -> anyhow::Result<()> {
    let native_options = ui::native_options("Synk Editor", "synk");
    ui::launch(native_options)?;
    Ok(())
}
