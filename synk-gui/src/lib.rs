mod components;

use iced::{
	application::Application,
	theme::{
		self,
		Theme,
	},
	widget::Container,
	Color,
	Command,
	Element,
};
use iced_aw::native::Split;

pub enum Mode {
	Insert,
	Normal,
	Replace,
}

#[derive(Debug, Clone)]

pub enum ThemeType {
	Dark,
	Light,
}

pub struct Synk {
	pub title: String,
	pub count: i32,
	pub is_dirty: bool,
	pub mode: Mode,
	pub sidebar_width: Option<u16>,
	pub sidebar_disabled: bool,
	pub theme: Theme,
}

pub struct StyleSheet {
	background: Color,
	foreground: Color,
	border_radius: f32,
	border_width: f32,
	border_color: Color,
}

pub struct EditorStyle;

impl iced::widget::container::StyleSheet for EditorStyle {
	type Style = Theme;

	fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
		iced::widget::container::Appearance {
			text_color: Some(StyleSheet::from_theme(style).foreground),
			background: Some(StyleSheet::from_theme(style).background.into()),
			border_radius: StyleSheet::from_theme(style).border_radius.into(),
			border_width: StyleSheet::from_theme(style).border_width,
			border_color: StyleSheet::from_theme(style).border_color.into(),
		}
	}
}

impl StyleSheet {
	pub fn from_theme(theme: &iced::Theme) -> StyleSheet {
		match theme {
			Theme::Dark => {
				StyleSheet {
					background: Color::from_rgb(30.0, 30.0, 46.0),
					foreground: Color::from_rgb(205.0, 214.0, 244.0),
					border_radius: 0.0,
					border_width: 1.0,
					border_color: Color::from_rgb(243.0, 139.0, 168.0),
				}
			}
			_ => {
				StyleSheet {
					background: Color::from_rgb(30.0, 30.0, 46.0),
					foreground: Color::from_rgb(205.0, 214.0, 244.0),
					border_radius: 10.0,
					border_width: 1.0,
					border_color: Color::from_rgb(243.0, 139.0, 168.0),
				}
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	OpenFile,
	SidebarResize(u16),
	ThemeChanged(ThemeType),
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
				theme: Theme::Dark,
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
			Message::SidebarResize(size) if size < 150 => self.sidebar_disabled = true,
			Message::SidebarResize(size) => self.sidebar_width = Some(size),
			Message::ThemeChanged(theme) => {
				self.theme = match theme {
					ThemeType::Dark => Theme::Dark,
					_ => Theme::Light,
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
