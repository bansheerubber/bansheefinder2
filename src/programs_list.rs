use iced::alignment::Horizontal;
use iced::border::radius;
use iced::widget::operation::move_cursor_to_end;
use iced::widget::scrollable::{Direction, Rail, Scrollbar};
use iced::widget::{self, column, container, scrollable, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Task};

use crate::autocomplete::default::DefaultFactory;
use crate::autocomplete::{CommandType, Factory, State};
use crate::style::{
	DARK_PURPLE, DISABLED_TEXT_COLOR, LIGHT_PURPLE, SCROLLBAR_PURPLE, SELECTED_TEXT_COLOR,
	TEXT_COLOR,
};

#[derive(Clone, Debug)]
pub enum Message {
	Autocomplete,
	SelectUp,
	SelectDown,
	Typed(String),
}

pub struct View {
	search: String,
	selected: Option<String>,
	state: Box<dyn State>,
	pub text_input: widget::Id,
}

impl View {
	pub fn new() -> Self {
		View {
			search: String::new(),
			selected: None,
			state: DefaultFactory::default().create(),
			text_input: widget::Id::unique(),
		}
	}

	pub fn start_program(&self) -> (String, Option<String>, CommandType) {
		self.state.get_command()
	}

	pub fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::Autocomplete => {
				let (search_text, selected_item) = self.state.autocomplete();
				let selected_item = if let Some(item) = selected_item {
					item
				} else {
					search_text.clone()
				};

				self.search = search_text;
				self.selected = Some(selected_item);
				move_cursor_to_end(self.text_input.clone())
			}
			Message::SelectUp => {
				let (search_text, selected_item) = self.state.select_up();
				let selected_item = if let Some(item) = selected_item {
					item
				} else {
					search_text.clone()
				};

				self.search = search_text;
				self.selected = Some(selected_item);
				move_cursor_to_end(self.text_input.clone())
			}
			Message::SelectDown => {
				let (search_text, selected_item) = self.state.select_down();
				let selected_item = if let Some(item) = selected_item {
					item
				} else {
					search_text.clone()
				};

				self.search = search_text;
				self.selected = Some(selected_item);
				move_cursor_to_end(self.text_input.clone())
			}
			Message::Typed(search) => {
				self.selected = None;
				self.search = search.clone();
				self.selected = None;
				self.state.update_search(search);
				Task::none()
			}
		}
	}

	pub fn view(&self) -> Element<'_, Message> {
		let mut scrollable_column = column![];

		let list = self.state.get_ui_list();
		if let Some(programs) = list.as_ref() {
			for autocomplete in programs.iter() {
				let moved_autocomplete = autocomplete.clone();

				scrollable_column = scrollable_column.push(
					container(
						text(autocomplete)
							.align_x(Horizontal::Left)
							.width(Length::Fill)
							.size(9),
					)
					.padding(3)
					.width(Length::Fill)
					.style(move |_| {
						if self.selected.is_some()
							&& &moved_autocomplete == self.selected.as_ref().unwrap()
						{
							container::Style {
								text_color: Some(TEXT_COLOR),
								background: Some(LIGHT_PURPLE.into()),
								border: Border::default(),
								shadow: Shadow::default(),
								snap: false,
							}
						} else {
							container::Style {
								text_color: Some(DISABLED_TEXT_COLOR),
								background: Some(DARK_PURPLE.into()),
								border: Border::default(),
								shadow: Shadow::default(),
								snap: false,
							}
						}
					}),
				);
			}
		}

		column![
			text_input("", &self.search)
				.id(self.text_input.clone())
				.size(15)
				.on_input(Message::Typed)
				.padding(7)
				.style(|_, _| {
					text_input::Style {
						background: DARK_PURPLE.into(),
						border: Border::default(),
						icon: Color::BLACK,
						placeholder: DISABLED_TEXT_COLOR,
						value: TEXT_COLOR,
						selection: SELECTED_TEXT_COLOR,
					}
				}),
			scrollable(scrollable_column)
				.direction(Direction::Vertical(
					Scrollbar::default().scroller_width(7.0).margin(1.0)
				))
				.style(|_, _| scrollable::Style {
					container: container::Style {
						text_color: Some(TEXT_COLOR),
						background: Some(Color::TRANSPARENT.into()),
						border: Border::default(),
						shadow: Shadow::default(),
						snap: false,
					},
					vertical_rail: Rail {
						background: Some(Color::TRANSPARENT.into()),
						border: Border::default(),
						scroller: scrollable::Scroller {
							background: SCROLLBAR_PURPLE.into(),
							border: Border::default()
								.width(0)
								.rounded(radius(5.0))
								.color(SCROLLBAR_PURPLE),
						}
					},
					horizontal_rail: Rail {
						background: Some(Color::TRANSPARENT.into()),
						border: Border::default(),
						scroller: scrollable::Scroller {
							background: SCROLLBAR_PURPLE.into(),
							border: Border::default()
								.width(0)
								.rounded(radius(5.0))
								.color(SCROLLBAR_PURPLE),
						}
					},
					gap: Some(Color::WHITE.into()),
					auto_scroll: scrollable::AutoScroll {
						background: SCROLLBAR_PURPLE.into(),
						border: Border::default(),
						shadow: Shadow::default(),
						icon: SCROLLBAR_PURPLE.into(),
					},
				}),
		]
		.padding(0)
		.align_x(Alignment::Center)
		.into()
	}
}
