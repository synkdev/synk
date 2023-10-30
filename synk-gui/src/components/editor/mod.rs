pub mod buffer;

use crate::Message;
use iced::{
	widget::{
		column,
		component,
		text,
		Component,
	},
	Element,
	Renderer,
};

pub struct Editor<Message> {
	pub fen: Message,
}

impl Editor<Message> {
	fn new() -> Self {
		Editor { fen: Message::OpenFile }
	}
}

pub enum Event {
	OpenFile,
}

pub fn editor() -> Editor<Message> {
	Editor::new()
}

impl Component<Message, Renderer> for Editor<Message> {
	type State = ();
	type Event = Event;

	fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::OpenFile => Some(Message::OpenFile),
		}
	}

	fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
		column![text("hello")].align_items(iced::Alignment::Center).spacing(10).into()
	}
}

impl<'a, Message> From<Editor<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
{
	fn from(editor: Editor<Message>) -> Self {
		component(editor)
	}
}
