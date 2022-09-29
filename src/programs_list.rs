use iced::{ Alignment, Column, Command, Container, Element, Length, Scrollable, Text, TextInput, alignment, container, scrollable, text_input };

use crate::autocomplete::{
	file_autocomplete,
	file_fuzzyfind,
};
use crate::launcher::launch_program;
use crate::style;
use crate::types;

#[derive(Clone, Debug)]
pub enum Message {
	Autocomplete,
	SelectUp,
	SelectDown,
	StartProgram,
	Typed(String),
}

#[derive(Clone, Copy, Debug)]
pub enum State {
	ShowAutocomplete,
	ShowFuzzy,
}

#[derive(Debug)]
pub struct View {
	input_state: text_input::State,
	programs: Option<types::SharedVec<String>>,
	programs_autocomplete: Option<Vec<String>>,
	programs_fuzzy: Option<Vec<String>>,
	scroll_state: scrollable::State,
	search: String,
	selected_program: Option<usize>,
	state: State,
}

impl View {
	pub fn new(programs: Option<types::SharedVec<String>>) -> Self {
		View {
			input_state: text_input::State::focused(),
			programs,
			programs_autocomplete: None,
			programs_fuzzy: None,
			scroll_state: scrollable::State::new(),
			search: String::new(),
			selected_program: None,
			state: State::ShowFuzzy,
		}
	}

	// cannot take self b/c of mutability rules
	fn get_list_from_state<'a>(
		state: State,
		autocomplete: &'a Option<Vec<String>>,
		fuzzy: &'a Option<Vec<String>>
	) -> &'a Option<Vec<String>> {
		match state {
			State::ShowAutocomplete => {
				&autocomplete
			},
			State::ShowFuzzy => {
				&fuzzy
			},
		}
	}

	// detect if we are in project list mode
	fn is_project_mode(search: &String) -> bool {
		if &search[0..12] == "open-project" {
			return true;
		} else {
			return false;
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Autocomplete => {
				if let Some(autocomplete) = self.programs_autocomplete.as_ref() {
					if autocomplete.len() == 0 {
						return Command::none();
					}

					self.state = State::ShowAutocomplete;
					self.selected_program = Some(0);
					self.search = autocomplete[0].clone();

					self.programs_autocomplete = file_autocomplete(&self.programs.as_ref().unwrap(), &self.search);
					self.programs_fuzzy = file_fuzzyfind(&self.programs.as_ref().unwrap(), &self.search);
				}
			},
			Message::StartProgram => {
				if let Some(index) = self.selected_program {
					let list = View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref();
					if let Some(list) = list {
						launch_program(list[index].clone());
					}
				} else {
					launch_program(self.search.clone());
				}
			},
			Message::SelectUp => {
				let list = View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref();
				if let None = list {
					self.selected_program = None;
				} else if let None = self.selected_program {
					self.selected_program = Some(0);
				} else if self.selected_program.unwrap() != list.unwrap().len() - 1 {
					self.selected_program = Some(self.selected_program.unwrap() + 1);
				}
			},
			Message::SelectDown => {
				if let None = self.selected_program {
					self.selected_program = Some(0);
				} else if self.selected_program.unwrap() != 0 {
					self.selected_program = Some(self.selected_program.unwrap() - 1);
				}
			},
			Message::Typed(search) => {
				self.state = State::ShowFuzzy;
				self.selected_program = None;
				self.search = search;
				self.input_state.move_cursor_to_end();

				self.programs_autocomplete = file_autocomplete(&self.programs.as_ref().unwrap(), &self.search);
				self.programs_fuzzy = file_fuzzyfind(&self.programs.as_ref().unwrap(), &self.search);
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scroll_state);
		let list = View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref();
		if let Some(programs) = list.as_ref() {
			let mut index = 0;
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
							if self.selected_program.is_some() && index == self.selected_program.unwrap() {
								Box::new(style::SelectedProgram) as Box<dyn container::StyleSheet>
							} else {
								Box::new(style::Program) as Box<dyn container::StyleSheet>
							}
						)
				);

				index += 1;
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
