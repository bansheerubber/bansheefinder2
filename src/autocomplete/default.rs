use crate::autocomplete::killall::KillallFactory;
use crate::autocomplete::open_project::OpenProjectFactory;
use crate::autocomplete::program_sorting::{ autocomplete, fuzzyfind, };
use crate::autocomplete::sudo::SudoFactory;
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
use crate::autocomplete::x11_forward::X11ForwardFactory;
use crate::path_interpreter::{
	ProgramFrequencyMap,
	get_programs,
	read_command_frequency,
};

pub struct DefaultState {
	active_list: ActiveList,
	autocomplete: Option<Autocomplete>,
	default_list: Option<Vec<String>>,
	factory: Box<dyn Factory>,
	fuzzyfind: List,
	passthrough: Option<Box<dyn State>>,
	passthrough_factories: Vec::<Box<dyn Factory>>,
	preamble: String,
	programs: Vec<String>,
	program_frequency: ProgramFrequencyMap,
	search: String,
	selected: Option<usize>,
}

impl Default for DefaultState {
	fn default() -> Self {
		DefaultState {
			active_list: ActiveList::default(),
			autocomplete: None,
			default_list: Some(
				vec![
					"open-project",
					"gitkraken",
					"okular",
					"backup",
					"steam",
					"warsow",
					"texstudio",
					"restart",
					"off",
				].iter()
				.map(|s| s.to_string())
				.collect::<Vec<String>>()
			),
			factory: Box::new(DefaultFactory),
			fuzzyfind: List::default(),
			passthrough: None,
			passthrough_factories: vec![
				Box::new(OpenProjectFactory),
				Box::new(SudoFactory),
				Box::new(KillallFactory),
				Box::new(X11ForwardFactory)
			],
			preamble: String::new(),
			programs: get_programs().unwrap(),
			program_frequency: read_command_frequency(),
			search: String::default(),
			selected: None,
		}
	}
}

impl State for DefaultState {
	fn get_factory(&self) -> &Box<dyn Factory> {
		&self.factory
	}

	fn get_replacement(&self) -> &String {
		&self.preamble
	}

	fn get_preamble(&self) -> &String {
		&self.preamble
	}

	fn get_active_list(&self) -> ActiveList {
		if self.search.len() == 0 {
			return ActiveList::FuzzyFinder;
		}

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
		if self.search.len() == 0 {
			return &self.default_list;
		}

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
		if self.search.len() == 0 && self.selected.is_some() {
			let command = self.default_list.as_ref().unwrap()[self.selected.unwrap()].clone();
			(command.clone(), Some(command), CommandType::Normal)
		} else {
			passthrough_command(
				&self.search,
				&self.search.split(' ').nth(0).unwrap().to_string(),
				CommandType::Normal,
				&self.passthrough
			)
		}
	}

	fn select_up(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.select_up());
		}

		let list = if self.search.len() == 0 {
			self.default_list.as_ref()
		} else {
			get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref()
		};

		if let None = list {
			self.selected = None;
		} else if self.selected.is_some() && self.selected.unwrap() != list.unwrap().len() - 1 {
			self.selected = Some(self.selected.unwrap() + 1);
		} else {
			self.selected = Some(0);
		}

		if let Some(index) = self.selected {
			if self.search.len() != 0 {
				self.search = list.unwrap()[index].clone();
			}

			(list.unwrap()[index].clone(), None)
		} else {
			(String::new(), None)
		}
	}

	fn select_down(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.select_down());
		}

		let list = if self.search.len() == 0 {
			self.default_list.as_ref()
		} else {
			get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref()
		};

		if let None = list {
			self.selected = None;
		} else if self.selected.is_some() && self.selected.unwrap() != 0 {
			self.selected = Some(self.selected.unwrap() - 1);
		} else {
			self.selected = Some(list.unwrap().len() - 1);
		}

		if let Some(index) = self.selected {
			if self.search.len() != 0 {
				self.search = list.unwrap()[index].clone();
			}

			(list.unwrap()[index].clone(), None)
		} else {
			(String::new(), None)
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct DefaultFactory;

impl Factory for DefaultFactory {
	fn should_create(&self, search: &String) -> bool {
		if search.len() == 0 {
			true
		} else {
			false
		}
	}

	fn create(&self) -> Box<dyn State> {
		Box::new(DefaultState::default())
	}
}
