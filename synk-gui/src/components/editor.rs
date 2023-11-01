use iced::{
	widget::{
		Container,
		Text,
	},
	Length,
	Renderer,
};

use crate::{
	Message,
	Synk,
};

impl Synk {
	pub fn view_editor(&self) -> iced::widget::Container<Message, Renderer> {
		Container::new(Text::new("Editor"))
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into()
	}
}
