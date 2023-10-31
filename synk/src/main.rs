use iced::{
	application::Application,
	Settings,
};
use synk_gui::Synk;

fn main() -> anyhow::Result<()> {
	Synk::run(Settings::default())?;

	Ok(())
}
