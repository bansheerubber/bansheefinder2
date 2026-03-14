mod autocomplete;
mod launcher;
mod path_interpreter;
mod programs_list;
mod style;
mod sudo_password;

use iced::{
	font::{Family, Stretch, Style, Weight},
	keyboard::{self, key::Named, Key},
	widget::{container, operation::focus},
	Border, Element, Font, Length, Shadow, Subscription, Task,
};
use style::{DARK_PURPLE, DISABLED_TEXT_COLOR, LIGHT_PURPLE};

enum CurrentView {
	ProgramList,
	SudoPassword,
}

#[derive(Debug)]
enum Message {
	KeyPressed(keyboard::Event),
	ProgramsListMessage(programs_list::Message),
	SudoPasswordViewMessage(sudo_password::Message),
}

struct Window {
	current_view: CurrentView,
	programs_list: programs_list::View,
	sudo_command: (Option<String>, Option<String>),
	sudo_password_view: sudo_password::View,
}

impl Window {
	fn boot() -> (Self, Task<Message>) {
		let programs_list = programs_list::View::new();
		let text_input_id = programs_list.text_input.clone();

		(
			Window {
				current_view: CurrentView::ProgramList,
				programs_list,
				sudo_command: (None, None),
				sudo_password_view: sudo_password::View::new(),
			},
			focus(text_input_id),
		)
	}

	fn subscription(&self) -> Subscription<Message> {
		keyboard::listen().map(|event| Message::KeyPressed(event))
	}

	fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::KeyPressed(key) => match key {
				keyboard::Event::KeyPressed {
					key: Key::Named(Named::ArrowDown),
					..
				} => self
					.programs_list
					.update(programs_list::Message::SelectUp)
					.map(move |message| Message::ProgramsListMessage(message)),
				keyboard::Event::KeyPressed {
					key: Key::Named(Named::Enter),
					..
				} => {
					if let CurrentView::ProgramList = self.current_view {
						match self.programs_list.start_program() {
							(command, base_command, autocomplete::CommandType::Normal)
							| (command, base_command, autocomplete::CommandType::OpenProject) => {
								launcher::launch_program(command, base_command);
								Task::none()
							}
							(command, base_command, autocomplete::CommandType::Sudo) => {
								self.current_view = CurrentView::SudoPassword;
								self.sudo_command = (Some(command), base_command);
								focus(self.sudo_password_view.text_input.clone())
							}
						}
					} else {
						let command = self.sudo_command.0.as_ref();
						let base_command = self.sudo_command.1.clone();
						launcher::launch_program_sudo(
							command.unwrap().clone(),
							base_command,
							self.sudo_password_view.get_password(),
						);

						Task::none()
					}
				}
				keyboard::Event::KeyPressed {
					key: Key::Named(Named::Escape),
					..
				} => {
					std::process::exit(0);
				}
				keyboard::Event::KeyPressed {
					key: Key::Named(Named::Tab),
					..
				} => self
					.programs_list
					.update(programs_list::Message::Autocomplete)
					.map(move |message| Message::ProgramsListMessage(message)),
				keyboard::Event::KeyPressed {
					key: Key::Named(Named::ArrowUp),
					..
				} => self
					.programs_list
					.update(programs_list::Message::SelectDown)
					.map(move |message| Message::ProgramsListMessage(message)),
				_ => Task::none(),
			},
			Message::ProgramsListMessage(message) => self
				.programs_list
				.update(message)
				.map(move |message| Message::ProgramsListMessage(message)),
			Message::SudoPasswordViewMessage(message) => self
				.sudo_password_view
				.update(message)
				.map(move |message| Message::SudoPasswordViewMessage(message)),
		}
	}

	fn view(&self) -> Element<'_, Message> {
		match self.current_view {
			CurrentView::ProgramList => container(
				self.programs_list
					.view()
					.map(move |message| Message::ProgramsListMessage(message)),
			)
			.height(Length::Fill)
			.padding(1)
			.style(|_| container::Style {
				text_color: Some(DISABLED_TEXT_COLOR),
				background: Some(DARK_PURPLE.into()),
				border: Border::default().width(1.0).color(LIGHT_PURPLE),
				shadow: Shadow::default(),
				snap: false,
			})
			.into(),
			CurrentView::SudoPassword => container(
				self.sudo_password_view
					.view()
					.map(move |message| Message::SudoPasswordViewMessage(message)),
			)
			.height(Length::Fill)
			.padding(1)
			.style(|_| container::Style {
				text_color: Some(DISABLED_TEXT_COLOR),
				background: Some(DARK_PURPLE.into()),
				border: Border::default().width(1.0).color(LIGHT_PURPLE),
				shadow: Shadow::default(),
				snap: false,
			})
			.into(),
		}
	}
}

fn main() {
	env_logger::init();

	// only open one finder at a time
	let pgrep_out = String::from_utf8(
		std::process::Command::new("pgrep")
			.arg("-xi")
			.arg("bansheefinder3")
			.output()
			.unwrap()
			.stdout,
	)
	.unwrap();

	if pgrep_out.trim().find('\n').is_none() {
		iced::application(Window::boot, Window::update, Window::view)
			.font(include_bytes!("../fonts/NotoSans-Regular.ttf"))
			.default_font(Font {
				family: Family::Name("Noto Sans"),
				weight: Weight::Medium,
				stretch: Stretch::Normal,
				style: Style::Normal,
			})
			.window_size((300.0, 200.0))
			.scale_factor(|_| 1.5)
			.decorations(false)
			.resizable(false)
			.antialiasing(false)
			.subscription(Window::subscription)
			.title("bansheefinder3")
			.run()
			.expect("Could not open bansheefinder3");
	} else {
		println!("already open");
	}
}
