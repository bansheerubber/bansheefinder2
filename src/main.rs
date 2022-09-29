mod autocomplete;
mod launcher;
mod path_interpreter;
mod program_search;
mod programs_list;
mod style;
mod types;

use iced::{ Alignment, Application, Column, Command, Container, Element, Length, Settings, Subscription, executor };
use std::sync::Arc;

use autocomplete::{
	file_autocomplete,
	file_fuzzyfind,
};
use path_interpreter::{
	get_programs,
};

#[derive(Debug)]
enum Message {
	EventOccurred(iced_native::Event),
	ProgramSearchMessage(program_search::Message),
	ProgramsListMessage(programs_list::Message),
}

struct Window {
	program_search: program_search::View,
	programs: Vec<String>,
	programs_autocomplete: types::SharedVec<String>,
	programs_fuzzy: types::SharedVec<String>,
	programs_list: programs_list::View,
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		(
			Window {
				program_search: program_search::View::new(),
				programs: get_programs().unwrap(),
				programs_autocomplete: Arc::new(Vec::new()),
				programs_fuzzy: Arc::new(Vec::new()),
				programs_list: programs_list::View::new(),
			},
			Command::none()
		)
	}

	fn title(&self) -> String {
		String::from("bansheefinder2")
	}

	fn subscription(&self) -> Subscription<Message> {
		iced_native::subscription::events().map(Message::EventOccurred)
	}

	fn update(&mut self, message: Message) -> Command<Self::Message> {
		match message {
			Message::EventOccurred(event) => {
				let event = if let iced_native::Event::Keyboard(event) = event {
					event
				} else {
					return Command::none();
				};

				let key_code = if let iced_native::keyboard::Event::KeyPressed { key_code, modifiers: _ } = event {
					key_code
				} else {
					return Command::none();
				};

				match key_code {
					iced_native::keyboard::KeyCode::Down => {
						self.programs_list.update(
							programs_list::Message::SelectUp
						).map(move |message| {
							Self::Message::ProgramsListMessage(message)
						})
					},
					iced_native::keyboard::KeyCode::Enter => {
						self.programs_list.update(
							programs_list::Message::StartProgram
						).map(move |message| {
							Self::Message::ProgramsListMessage(message)
						})
					},
					iced_native::keyboard::KeyCode::Escape => {
						std::process::exit(0);
					},
					iced_native::keyboard::KeyCode::Tab => {
						if self.programs_autocomplete.len() > 0 {
							self.update( // send through Application update so that the program list gets its state updated
								Self::Message::ProgramSearchMessage(
									program_search::Message::Autocomplete(self.programs_autocomplete[0].clone())
								)
							)
						} else {
							Command::none()
						}
					},
					iced_native::keyboard::KeyCode::Up => {
						self.programs_list.update(
							programs_list::Message::SelectDown
						).map(move |message| {
							Self::Message::ProgramsListMessage(message)
						})
					},
					_ => Command::none(),
				}
			},
			Message::ProgramSearchMessage(message) => {
				if let program_search::Message::Typed(search) = &message {
					// generate fuzzy and autocomplete lists
					self.programs_autocomplete = Arc::new(file_autocomplete(&self.programs, &search).unwrap());
					self.programs_fuzzy = Arc::new(file_fuzzyfind(&self.programs, &search).unwrap());

					self.programs_list.update(
						programs_list::Message::UpdateSearch(
							search.clone(),
							Some(self.programs_fuzzy.clone()),
						)
					).map(move |message| {
						Self::Message::ProgramsListMessage(message)
					});
				} else if let program_search::Message::Autocomplete(search) = &message {
					// generate fuzzy and autocomplete lists
					self.programs_autocomplete = Arc::new(file_autocomplete(&self.programs, &search).unwrap());
					self.programs_fuzzy = Arc::new(file_fuzzyfind(&self.programs, &search).unwrap());

					self.programs_list.update(
						programs_list::Message::Autocomplete(
							search.clone(),
							Some(self.programs_autocomplete.clone()),
						)
					).map(move |message| {
						Self::Message::ProgramsListMessage(message)
					});
				}

				self.program_search.update(message).map(move |message| {
					Self::Message::ProgramSearchMessage(message)
				})
			},
			Message::ProgramsListMessage(message) => {
				self.programs_list.update(message).map(move |message| {
					Self::Message::ProgramsListMessage(message)
				})
			},
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		Container::new(
			Column::new()
				.padding(0)
				.align_items(Alignment::Center)
				.push(
					self.program_search.view().map(move |message| {
						Self::Message::ProgramSearchMessage(message)
					})
				)
				.push(
					self.programs_list.view().map(move |message| {
						Self::Message::ProgramsListMessage(message)
					})
				)
			)
			.height(Length::Fill)
			.padding(1)
			.style(style::Application)
			.into()
	}
}

fn main() {
	// only open one finder at a time
	let pgrep_out = String::from_utf8(
		std::process::Command::new("pgrep")
		.arg("-xi")
		.arg("bansheefinder2")
		.output()
		.unwrap()
		.stdout
	).unwrap();

	if pgrep_out.trim().find('\n').is_none() {
		println!(
			"{:?}",
				Window::run(Settings {
				antialiasing: false,
				text_multithreading: false,
				window: iced::window::Settings {
					decorations: false,
					resizable: false,
					size: (300, 200),
					..iced::window::Settings::default()
				},
				..Settings::default()
			})
		);
	} else {
		println!("already open");
	}
}
