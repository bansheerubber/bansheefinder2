use iced::{ Alignment, Column, Command, Container, Element, Length, Scrollable, Text, TextInput, alignment, container, scrollable, text_input };

use crate::autocomplete2::{ Factory, State };
use crate::autocomplete2::default::DefaultFactory;
use crate::launcher::launch_program;
use crate::style;

#[derive(Clone, Debug)]
pub enum Message {
	Autocomplete,
	SelectUp,
	SelectDown,
	StartProgram,
	Typed(String),
}

pub struct View {
	input_state: text_input::State,
	scroll_state: scrollable::State,
	search: String,
	selected: Option<String>,
	state: Box<dyn State>,
}

impl View {
	pub fn new() -> Self {
		View {
			input_state: text_input::State::focused(),
			scroll_state: scrollable::State::new(),
			search: String::new(),
			selected: None,
			state: DefaultFactory::default().create(),
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Autocomplete => {
				self.search = self.state.autocomplete();
				self.selected = Some(self.search.clone());
				self.input_state.move_cursor_to_end();
			},
			Message::StartProgram => {
				launch_program(self.state.get_command());
			},
			Message::SelectUp => {
				let (search_text, selected_item) = self.state.select_up();
				let selected_item = if let Some(item) = selected_item {
					item
				} else {
					search_text.clone()
				};

				self.search = search_text;
				self.selected = Some(selected_item);
				self.input_state.move_cursor_to_end();
			},
			Message::SelectDown => {
				let (search_text, selected_item) = self.state.select_down();
				let selected_item = if let Some(item) = selected_item {
					item
				} else {
					search_text.clone()
				};

				self.search = search_text;
				self.selected = Some(selected_item);
				self.input_state.move_cursor_to_end();
			},
			Message::Typed(search) => {
				self.selected = None;
				self.search = search.clone();
				self.selected = None;
				self.state.update_search(search);
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scroll_state);

		let list = self.state.get_ui_list();
		if let Some(programs) = list.as_ref() {
			for autocomplete in programs.iter() {
				scrollable = scrollable.push(
					Container::new(
						Text::new(autocomplete)
							.horizontal_alignment(alignment::Horizontal::Left)
							.width(Length::Fill)
							.size(10)
					)
						.padding(3)
						.width(Length::Fill)
						.style(
							if self.selected.is_some() && autocomplete == self.selected.as_ref().unwrap() {
								Box::new(style::SelectedProgram) as Box<dyn container::StyleSheet>
							} else {
								Box::new(style::Program) as Box<dyn container::StyleSheet>
							}
						)
				);
			}
		}

		Column::new()
			.padding(0)
			.align_items(Alignment::Center)
			.push(
				TextInput::new(
					&mut self.input_state,
					"",
					&self.search,
					Message::Typed,
				)
					.size(15)
					.padding(7)
					.style(style::SearchInput)
			)
			.push(
				scrollable
			)
			.into()

	}
}
