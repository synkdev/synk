use iced::widget::{
	container,
	text,
};

use crate::{
	Message,
	Synk,
};

impl Synk {
	pub fn view_editor(&self) -> iced::Element<Message> {
		container(text("editor")).padding(10).into()
	}
}
