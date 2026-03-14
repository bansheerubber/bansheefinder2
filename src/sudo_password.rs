use iced::{
	widget::{self, column, container, text, text_input, Space},
	Alignment, Border, Color, Element, Length, Padding, Task,
};

use crate::style::{DARK_PURPLE, DISABLED_TEXT_COLOR, SELECTED_TEXT_COLOR, TEXT_COLOR};

#[derive(Clone, Debug)]
pub enum Message {
	Typed(String),
}

pub struct View {
	password: String,
	pub text_input: widget::Id,
}

impl View {
	pub fn new() -> Self {
		View {
			password: String::new(),
			text_input: widget::Id::unique(),
		}
	}

	pub fn get_password(&self) -> String {
		self.password.clone()
	}

	pub fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::Typed(password) => {
				self.password = password;
			}
		}

		Task::none()
	}

	pub fn view(&self) -> Element<'_, Message> {
		column![
			Space::new().height(Length::Fixed(5.0)),
			container(text("Password").size(14).width(Length::Fill))
				.width(Length::Fill)
				.padding(Padding::default().left(7)), // [0, 0, 0, 7]
			text_input("", &self.password)
				.id(self.text_input.clone())
				.size(15)
				.secure(true)
				.on_input(Message::Typed)
				.padding(Padding::default().top(4).right(7).bottom(4).left(7)) // [4, 7, 4, 7]
				.style(|_, _| text_input::Style {
					background: DARK_PURPLE.into(),
					border: Border::default(),
					icon: Color::BLACK,
					placeholder: DISABLED_TEXT_COLOR,
					value: TEXT_COLOR,
					selection: SELECTED_TEXT_COLOR,
				})
		]
		.padding(0)
		.align_x(Alignment::Center)
		.height(Length::Fill)
		.into()
	}
}
