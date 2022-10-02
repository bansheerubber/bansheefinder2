use iced::{ Alignment, Column, Command, Container, Element, Length, Space, Text, TextInput, text_input };

use crate::style;

#[derive(Clone, Debug)]
pub enum Message {
	Typed(String),
}

pub struct View {
	input_state: text_input::State,
	password: String,
}

impl View {
	pub fn new() -> Self {
		View {
			input_state: text_input::State::focused(),
			password: String::new(),
		}
	}

	pub fn get_password(&self) -> String {
		self.password.clone()
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Typed(password) => {
				self.password = password;
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		Column::new()
			.padding(0)
			.align_items(Alignment::Center)
			.push(
				Space::new(Length::Units(0), Length::Units(5))
			)
			.push(
				Container::new(
					Text::new("Password")
						.size(14)
						.width(Length::Fill)
				)
					.width(Length::Fill)
					.padding([0, 0, 0, 7])
			)
			.push(
				TextInput::new(
					&mut self.input_state,
					"",
					&self.password,
					Message::Typed,
				)
					.size(15)
					.padding([4, 7, 4, 7])
					.password()
					.style(style::PasswordInput)
			)
			.height(Length::Fill)
			.into()

	}
}
