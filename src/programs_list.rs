use iced::{ Alignment, Column, Command, Container, Element, Length, Scrollable, Text, TextInput, alignment, container, scrollable, text_input };

use crate::autocomplete::{
	file_autocomplete,
	file_fuzzyfind,
	project_autocomplete,
	project_fuzzyfind,
};
use crate::launcher::launch_program;
use crate::path_interpreter::get_projects;
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
	projects: Option<Vec<String>>,
	programs_autocomplete: Option<Vec<String>>,
	programs_fuzzy: Option<Vec<String>>,
	projects_autocomplete: Option<Vec<String>>,
	projects_fuzzy: Option<Vec<String>>,
	scroll_state: scrollable::State,
	search: String,
	selected: Option<usize>,
	state: State,
}

impl View {
	pub fn new(programs: Option<types::SharedVec<String>>) -> Self {
		View {
			input_state: text_input::State::focused(),
			programs,
			projects: get_projects(),
			programs_autocomplete: None,
			programs_fuzzy: None,
			projects_autocomplete: None,
			projects_fuzzy: None,
			scroll_state: scrollable::State::new(),
			search: String::new(),
			selected: None,
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
	fn get_project(search: &String) -> Option<String> {
		if search.len() >= 13 && &search[0..12] == "open-project" {
			Some(search[13..].to_string())
		} else {
			None
		}
	}

	fn get_command(search: &String, index: &Option<usize>, list: Option<&Vec<String>>) -> String {
		if let Some(index) = index {
			let list = if list.is_none() {
				return String::from("");
			} else {
				list.unwrap()
			};

			if View::get_project(search).is_some() {
				format!("open-project {}", list[*index])
			} else {
				list[*index].clone()
			}
		} else {
			search.clone()
		}
	}

	pub fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Autocomplete => {
				let list = if View::get_project(&self.search).is_some() {
					self.projects_autocomplete.as_ref()
				} else {
					self.programs_autocomplete.as_ref()
				};

				if let Some(autocomplete) = list {
					if autocomplete.len() == 0 {
						return Command::none();
					}

					self.state = State::ShowAutocomplete;
					self.selected = Some(0);

					if View::get_project(&self.search).is_some() {
						self.search = format!("open-project {}", autocomplete[0]);
					} else {
						self.search = autocomplete[0].clone();
					}

					if let Some(project) = View::get_project(&self.search) {
						self.projects_autocomplete = project_autocomplete(&self.projects.as_ref().unwrap(), &project);
						self.projects_fuzzy = project_fuzzyfind(&self.projects.as_ref().unwrap(), &project);
					} else {
						self.programs_autocomplete = file_autocomplete(&self.programs.as_ref().unwrap(), &self.search);
						self.programs_fuzzy = file_fuzzyfind(&self.programs.as_ref().unwrap(), &self.search);
					}
				}
			},
			Message::StartProgram => {
				let list = if View::get_project(&self.search).is_some() {
					View::get_list_from_state(self.state, &self.projects_autocomplete, &self.projects_fuzzy).as_ref()
				} else {
					View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref()
				};

				launch_program(View::get_command(&self.search, &self.selected, list));
			},
			Message::SelectUp => {
				let list = if View::get_project(&self.search).is_some() {
					View::get_list_from_state(self.state, &self.projects_autocomplete, &self.projects_fuzzy).as_ref()
				} else {
					View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref()
				};

				if let None = list {
					self.selected = None;
				} else if let None = self.selected {
					self.selected = Some(0);
				} else if self.selected.unwrap() != list.unwrap().len() - 1 {
					self.selected = Some(self.selected.unwrap() + 1);
				}
			},
			Message::SelectDown => {
				if let None = self.selected {
					self.selected = Some(0);
				} else if self.selected.unwrap() != 0 {
					self.selected = Some(self.selected.unwrap() - 1);
				}
			},
			Message::Typed(search) => {
				self.state = State::ShowFuzzy;
				self.selected = None;
				self.search = search;
				self.input_state.move_cursor_to_end();

				if let Some(project) = View::get_project(&self.search) {
					self.projects_autocomplete = project_autocomplete(&self.projects.as_ref().unwrap(), &project);
					self.projects_fuzzy = project_fuzzyfind(&self.projects.as_ref().unwrap(), &project);
				} else {
					self.programs_autocomplete = file_autocomplete(&self.programs.as_ref().unwrap(), &self.search);
					self.programs_fuzzy = file_fuzzyfind(&self.programs.as_ref().unwrap(), &self.search);
				}
			},
		}

		Command::none()
	}

	pub fn view(&mut self) -> Element<Message> {
		let mut scrollable = Scrollable::new(&mut self.scroll_state);

		let list = if View::get_project(&self.search).is_some() {
			View::get_list_from_state(self.state, &self.projects_autocomplete, &self.projects_fuzzy).as_ref()
		} else {
			View::get_list_from_state(self.state, &self.programs_autocomplete, &self.programs_fuzzy).as_ref()
		};

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
							if self.selected.is_some() && index == self.selected.unwrap() {
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
