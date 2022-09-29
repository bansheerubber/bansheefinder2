use iced::{ Background, Color, container, scrollable, text_input };

pub const DARK_PURPLE: Color = Color::from_rgb(
	0x1E as f32 / 255.0,
	0x12 as f32 / 255.0,
	0x1E as f32 / 255.0,
);

pub const TEXT_COLOR: Color = Color::from_rgb(
	0xB7 as f32 / 255.0,
	0xAC as f32 / 255.0,
	0xB7 as f32 / 255.0,
);

pub const DISABLED_TEXT_COLOR: Color = Color::from_rgb(
	0x80 as f32 / 255.0,
	0x78 as f32 / 255.0,
	0x80 as f32 / 255.0,
);

pub const LIGHT_PURPLE: Color = Color::from_rgb(
	0x38 as f32 / 255.0,
	0x26 as f32 / 255.0,
	0x3F as f32 / 255.0,
);

// style for program list search
pub struct SearchInput;
impl text_input::StyleSheet for SearchInput {
	fn active(&self) -> text_input::Style {
		text_input::Style {
			background: Background::Color(DARK_PURPLE),
			border_color: DARK_PURPLE,
			border_radius: 0.0,
			border_width: 1.0,
		}
	}

	fn value_color(&self) -> Color {
		TEXT_COLOR
	}

	fn placeholder_color(&self) -> Color {
		DISABLED_TEXT_COLOR
	}

	fn focused(&self) -> text_input::Style {
		self.active()
	}

	fn hovered(&self) -> text_input::Style {
		self.focused()
	}

	fn selection_color(&self) -> Color {
		LIGHT_PURPLE
	}
}

// style for the program list
pub struct ProgramList;
impl scrollable::StyleSheet for ProgramList {
	fn active(&self) -> scrollable::Scrollbar {
		scrollable::Scrollbar {
			background: Some(Background::Color(Color::TRANSPARENT)),
			border_color: Color::TRANSPARENT,
			border_radius: 0.0,
			border_width: 0.0,
			scroller: scrollable::Scroller {
				border_color: DARK_PURPLE,
				border_radius: 5.0,
				border_width: 0.0,
				color: LIGHT_PURPLE
			},
		}
	}

	fn hovered(&self) -> scrollable::Scrollbar {
		self.active()
	}

	fn dragging(&self) -> scrollable::Scrollbar {
		self.active()
	}
}

// style for the application
pub struct Application;
impl container::StyleSheet for Application {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(DARK_PURPLE)),
			border_color: LIGHT_PURPLE,
			border_radius: 0.0,
			border_width: 1.0,
			text_color: Some(DISABLED_TEXT_COLOR),
			..container::Style::default()
		}
	}
}

// style for a program in the program list
pub struct Program;
impl container::StyleSheet for Program {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(DARK_PURPLE)),
			text_color: Some(DISABLED_TEXT_COLOR),
			..container::Style::default()
		}
	}
}

// style for a selected program in the program list
pub struct SelectedProgram;
impl container::StyleSheet for SelectedProgram {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(LIGHT_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}
