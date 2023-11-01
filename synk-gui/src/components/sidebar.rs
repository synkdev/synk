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
	pub fn view_sidebar(&self) -> iced::widget::Container<Message, Renderer> {
		Container::new(Text::new("Sidebar"))
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into()
	}
}
