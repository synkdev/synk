mod components;

use iced::{
	application::Application,
	widget::Container,
	Command,
	Element,
	Theme,
};
use iced_aw::native::Split;

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
	pub sidebar_disabled: bool,
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
				sidebar_disabled: false,
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
			Message::SidebarResize(size) => {
				if size < 100 {
					self.sidebar_disabled = true
				} else {
					self.sidebar_width = Some(size)
				}
			}
		}
		Command::none()
	}
	fn view(&self) -> Element<Message> {
		if !self.sidebar_disabled {
			Split::new(
				self.view_sidebar(),
				self.view_editor(),
				self.sidebar_width,
				iced_aw::split::Axis::Vertical,
				Message::SidebarResize,
			)
			.into()
		} else {
			Container::new(self.view_editor()).into()
		}
	}

	fn theme(&self) -> Self::Theme {
		Theme::Dark
	}
}
