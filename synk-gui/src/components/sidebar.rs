use iced::widget::{
	container,
	text,
};

use crate::{
	Message,
	Synk,
};

impl Synk {
	pub fn view_sidebar(&self) -> iced::Element<Message> {
		container(text("sidebar")).padding(10).into()
	}
}
