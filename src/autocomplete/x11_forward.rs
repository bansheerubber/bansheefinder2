use crate::autocomplete::program_sorting::{ autocomplete, fuzzyfind, };
use crate::autocomplete::types::{
	ActiveList,
	Autocomplete,
	CommandType,
	Factory,
	List,
	State,
	get_ui_list,
	handle_update_placeholder,
	passthrough_command,
	passthrough_string,
};
use crate::path_interpreter::{
	ProgramFrequencyMap,
	get_programs,
	read_command_frequency,
};

pub struct X11ForwardState {
	active_list: ActiveList,
	autocomplete: Option<Autocomplete>,
	factory: Box<dyn Factory>,
	fuzzyfind: List,
	passthrough: Option<Box<dyn State>>,
	passthrough_factories: Vec::<Box<dyn Factory>>,
	preamble: String,
	programs: Vec<String>,
	program_frequency: ProgramFrequencyMap,
	replacement: String,
	search: String,
	selected: Option<usize>,
}

impl Default for X11ForwardState {
	fn default() -> Self {
		X11ForwardState {
			active_list: ActiveList::default(),
			autocomplete: None,
			factory: Box::new(X11ForwardFactory),
			fuzzyfind: List::default(),
			passthrough: None,
			passthrough_factories: vec![],
			preamble: String::from("ssh me@$(ping bansheestation -c 1 -q -W 1 | grep -q \"1 received\" && echo \"bansheestation\" || echo \"bansheestation-alt\") "),
			programs: get_programs().unwrap(),
			program_frequency: read_command_frequency(),
			replacement: String::from("!"),
			search: String::default(),
			selected: None,
		}
	}
}

impl State for X11ForwardState {
	fn get_factory(&self) -> &Box<dyn Factory> {
		&self.factory
	}

	fn get_replacement(&self) -> &String {
		&self.replacement
	}

	fn get_preamble(&self) -> &String {
		&self.preamble
	}

	fn get_active_list(&self) -> ActiveList {
		if let Some(passthrough) = self.passthrough.as_ref() {
			passthrough.get_active_list()
		} else {
			self.active_list
		}
	}

	fn get_autocomplete_list(&self) -> &List {
		if let Some(passthrough) = self.passthrough.as_ref() {
			passthrough.get_autocomplete_list()
		} else if let Some(autocomplete) = self.autocomplete.as_ref() {
			&autocomplete.list
		} else {
			&None
		}
	}

	fn get_fuzzyfinder_list(&self) -> &List {
		if let Some(passthrough) = self.passthrough.as_ref() {
			passthrough.get_fuzzyfinder_list()
		} else {
			&self.fuzzyfind
		}
	}

	fn update_search(&mut self, search: String) {
		if handle_update_placeholder(&search, &mut self.passthrough, &self.passthrough_factories) {
			let mut search = search;
			search.drain(0..self.passthrough.as_ref().unwrap().get_replacement().len());

			self.selected = None;
			self.search = self.passthrough.as_ref().unwrap().get_preamble().clone();

			self.passthrough.as_mut().unwrap().update_search(search);
		} else {
			self.search = search;
			self.active_list = ActiveList::FuzzyFinder;
			self.selected = None;

			self.autocomplete = autocomplete(&self.programs, &self.program_frequency, &self.search);
			self.fuzzyfind = fuzzyfind(&self.programs, &self.program_frequency, &self.search);
		}
	}

	fn autocomplete(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.autocomplete());
		}

		self.active_list = ActiveList::Autocomplete;
		if let Some(list) = self.autocomplete.as_ref() {
			self.search = list.common_start.clone();
		}

		self.autocomplete = autocomplete(&self.programs, &self.program_frequency, &self.search);
		self.fuzzyfind = fuzzyfind(&self.programs, &self.program_frequency, &self.search);

		self.selected = Some(0);

		(self.search.clone(), None)
	}

	fn get_command(&self) -> (String, Option<String>, CommandType) {
		passthrough_command(
			&self.search,
			&self.search[1..].to_string(),
			CommandType::Normal,
			&self.passthrough
		)
	}

	fn select_up(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.select_up());
		}

		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if self.selected.is_some() && self.selected.unwrap() != list.unwrap().len() - 1 {
			self.selected = Some(self.selected.unwrap() + 1);
		} else {
			self.selected = Some(0);
		}

		if let Some(index) = self.selected {
			self.search = list.unwrap()[index].clone();
			(list.unwrap()[index].clone(), None)
		} else {
			(String::new(), None)
		}
	}

	fn select_down(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.select_down());
		}

		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if self.selected.is_some() && self.selected.unwrap() != 0 {
			self.selected = Some(self.selected.unwrap() - 1);
		} else {
			self.selected = Some(list.unwrap().len() - 1);
		}

		if let Some(index) = self.selected {
			self.search = list.unwrap()[index].clone();
			(list.unwrap()[index].clone(), None)
		} else {
			(String::new(), None)
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct X11ForwardFactory;

impl Factory for X11ForwardFactory {
	fn should_create(&self, search: &String) -> bool {
		if search.len() >= 1 && search.chars().nth(0).unwrap() == '!' {
			true
		} else {
			false
		}
	}

	fn create(&self) -> Box<dyn State> {
		Box::new(X11ForwardState::default())
	}
}
