mod components;

use iced::{
	application::Application,
	widget::{
		container,
		Container,
		Text,
	},
	Command,
	Element,
	Length,
	Theme,
};
use iced_aw::Split;

pub enum Mode {
	Insert,
	Normal,
	Replace,
}

pub struct Synk {
	pub title: String,
	pub count: i32,
	pub is_dirty: bool,
	pub mode: Mode,
	pub sidebar_width: Option<u16>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	OpenFile,
	SidebarResize(u16),
}

impl Application for Synk {
	type Message = Message;
	type Flags = ();
	type Theme = Theme;
	type Executor = iced::executor::Default;

	fn new(_flags: Self::Flags) -> (Synk, Command<Message>) {
		(
			Synk {
				title: String::from("Synk"),
				count: 0,
				is_dirty: false,
				mode: Mode::Normal,
				sidebar_width: Some(300),
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from("Synk")
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::OpenFile => (),
			Message::SidebarResize(size) => self.sidebar_width = Some(size),
		}
		Command::none()
	}
	fn view(&self) -> Element<Message> {
		Split::new(
			self.view_sidebar().into(),
			self.view_editor().into(),
			self.sidebar_width,
			iced_aw::split::Axis::Vertical,
			Message::SidebarResize,
		)
		.into()
	}

	fn theme(&self) -> Self::Theme {
		Theme::Light
	}
}
