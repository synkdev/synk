use iced::Application;

pub struct Synk {
	pub title: String,
	pub count: i32,
}

pub enum Message {}

impl Application for Synk {
	type Message = Message;

	fn new(_flags: Self::flags) -> Self {
		Synk { title: String::from("Synk"), count: 0 }
	}

	fn title(&self) -> String {
		String::from("Synk")
	}
}
