use iced::{
	widget::{
		Container,
		Text,
	},
	Length,
};

use crate::{
	Message,
	Synk,
};

impl Synk {
	pub fn view_editor(&self) -> iced::Element<Message> {
		Container::new(Text::new("First"))
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into()
	}
}
