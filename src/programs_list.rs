use iced::{ Command, Container, Element, Length, Scrollable, Text, alignment, container, scrollable };

use crate::launcher::launch_program;
use crate::style;
use crate::types;

#[derive(Debug)]
pub struct View {
	programs_autocomplete: Option<types::SharedVec<String>>,
	programs_fuzzy: Option<types::SharedVec<String>>,
	scroll: scrollable::State,
	search: String,
	selected_program: Option<usize>,
	state: State,
}

#[derive(Debug)]
pub enum Message {
	Autocomplete(String, Option<types::SharedVec<String>>),
	SelectUp,
	SelectDown,
	StartProgram,
	UpdateSearch(String, Option<types::SharedVec<String>>),
}

#[derive(Clone, Copy, Debug)]
pub enum State {
	ShowAutocomplete,
	ShowFuzzy,
}

impl View {
	pub fn new() -> Self {
		View {
			programs_autocomplete: None,
			programs_fuzzy: None,
			scroll: scrollable::State::new(),
			search: String::new(),
			selected_program: None,
			state: State::ShowFuzzy,
		}
	}

	// cannot take self b/c of mutability rules
	fn get_list_from_state<'a>(
		state: State,
		autocomplete: &'a Option<types::SharedVec<String>>,
		fuzzy: &'a Option<types::SharedVec<String>>
	) -> &'a Option<types::SharedVec<String>> {
		match state {
			State::ShowAutocomplete => {
				&autocomplete
			},
			State::ShowFuzzy => {
				&fuzzy
			},
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Autocomplete(search, autocomplete) => {
				self.state = State::ShowAutocomplete;
				self.programs_autocomplete = autocomplete;
				self.selected_program = Some(0);
				self.search = search;
			},
			Message::UpdateSearch(search, fuzzy) => {
				self.state = State::ShowFuzzy;
				self.programs_fuzzy = fuzzy;
				self.selected_program = None;
				self.search = search;
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
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scroll);
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

		scrollable.into()
	}
}
