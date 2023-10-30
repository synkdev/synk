mod components;

use crate::components::editor::editor;
use iced::{
	application::Application,
	widget::container,
	Alignment,
	Command,
	Element,
	Length,
	Theme,
};

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
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	OpenFile,
}

impl Application for Synk {
	type Message = Message;
	type Flags = ();
	type Theme = Theme;
	type Executor = iced::executor::Default;

	fn new(_flags: Self::Flags) -> (Synk, Command<Message>) {
		(
			Synk { title: String::from("Synk"), count: 0, is_dirty: false, mode: Mode::Normal },
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from("Synk")
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::OpenFile => (),
		}
		Command::none()
	}
	fn view(&self) -> Element<Message> {
		container(editor()).padding(20).height(Length::Fill).center_y().center_x().into()
	}
}
