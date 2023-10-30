use iced::{
	widget::{
		button,
		column,
		text,
	},
	Alignment,
	Application,
	Command,
	Element,
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
	Increment,
	Decrement,
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
			Message::Decrement => self.count -= 1,
			Message::Increment => self.count += 1,
		}
		Command::none()
	}
	fn view(&self) -> Element<Message> {
		column![
			button("Increment").on_press(Message::Increment),
			text(self.count).size(50),
			button("Decrement").on_press(Message::Decrement)
		]
		.padding(20)
		.align_items(Alignment::Center)
		.into()
	}
}
