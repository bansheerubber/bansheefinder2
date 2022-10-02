mod autocomplete;
mod launcher;
mod path_interpreter;
mod programs_list;
mod style;
mod sudo_password;

use iced::{ Application, Command, Container, Element, Length, Settings, Subscription, executor };

enum CurrentView {
	ProgramList,
	SudoPassword,
}

#[derive(Debug)]
enum Message {
	EventOccurred(iced_native::Event),
	ProgramsListMessage(programs_list::Message),
	SudoPasswordViewMessage(sudo_password::Message),
}

struct Window {
	current_view: CurrentView,
	programs_list: programs_list::View,
	sudo_command: Option<String>,
	sudo_password_view: sudo_password::View,
}

impl Application for Window {
	type Message = Message;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		(
			Window {
				current_view: CurrentView::ProgramList,
				programs_list: programs_list::View::new(),
				sudo_command: None,
				sudo_password_view: sudo_password::View::new(),
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
						if let CurrentView::ProgramList = self.current_view {
							match self.programs_list.start_program() {
								(command, autocomplete::CommandType::Normal) | (command, autocomplete::CommandType::OpenProject) => {
									launcher::launch_program(command);
								},
								(command, autocomplete::CommandType::Sudo) => {
									self.current_view = CurrentView::SudoPassword;
									self.sudo_command = Some(command);
								},
							}
						} else {
							launcher::launch_program_sudo(self.sudo_command.as_ref().unwrap().clone(), self.sudo_password_view.get_password());
						}

						Command::none()
					},
					iced_native::keyboard::KeyCode::Escape => {
						std::process::exit(0);
					},
					iced_native::keyboard::KeyCode::Tab => {
						self.programs_list.update(programs_list::Message::Autocomplete).map(move |message| {
							Self::Message::ProgramsListMessage(message)
						})
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
			Message::ProgramsListMessage(message) => {
				self.programs_list.update(message).map(move |message| {
					Self::Message::ProgramsListMessage(message)
				})
			},
			Message::SudoPasswordViewMessage(message) => {
				self.sudo_password_view.update(message).map(move |message| {
					Self::Message::SudoPasswordViewMessage(message)
				})
			},
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.current_view {
			CurrentView::ProgramList => {
				Container::new(
					self.programs_list.view().map(move |message| {
						Self::Message::ProgramsListMessage(message)
					})
				)
					.height(Length::Fill)
					.padding(1)
					.style(style::Application)
					.into()
			},
			CurrentView::SudoPassword => {
				Container::new(
					self.sudo_password_view.view().map(move |message| {
						Self::Message::SudoPasswordViewMessage(message)
					})
				)
					.height(Length::Fill)
					.padding(1)
					.style(style::Application)
					.into()
			}
		}
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
