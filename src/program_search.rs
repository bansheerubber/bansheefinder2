use iced::{ Command, Element, TextInput, text_input };

use crate::style;

#[derive(Debug)]
pub struct View {
	input: text_input::State,
	search: String,
}

#[derive(Clone, Debug)]
pub enum Message {
	Autocomplete(String),
	Typed(String),
}

impl View {
	pub fn new() -> Self {
		View {
			input: text_input::State::focused(),
			search: String::new(),
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Autocomplete(search) => {
				self.search = search;
				self.input.move_cursor_to_end();
			},
			Message::Typed(search) => {
				self.search = search;
			},
		}
		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		TextInput::new(
			&mut self.input,
			"",
			&self.search,
			Message::Typed,
		)
			.size(15)
			.padding(7)
			.style(style::SearchInput)
			.into()
	}
}
