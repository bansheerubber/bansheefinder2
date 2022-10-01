use crate::autocomplete::types::{
	ActiveList,
	Factory,
	List,
	State,
	get_ui_list,
};
use crate::autocomplete::open_project::OpenProjectFactory;
use crate::path_interpreter::get_programs;

fn fuzzyfind(programs: &Vec<String>, search: &String) -> Option<Vec<String>> {
	let mut output = programs.iter()
		.fold(Vec::new(), |mut acc, program| {
			if program.find(search).is_some() {
				acc.push(program.clone())
			}

			acc
		});

	output.sort_by(
		|a, b| {
			a.len().cmp(&b.len())
		}
	);

	Some(output)
}

fn autocomplete(programs: &Vec<String>, search: &String) -> Option<Vec<String>> {
	let mut output = programs.iter()
		.fold(Vec::new(), |mut acc, program| {
			if let Some(index) = program.find(search) {
				if index == 0 {
					acc.push(program.clone())
				}
			}

			acc
		});

	output.sort_by(
		|a, b| {
			a.len().cmp(&b.len())
		}
	);

	Some(output)
}

pub struct DefaultState {
	active_list: ActiveList,
	autocomplete: List,
	factory: Box<dyn Factory>,
	fuzzyfind: List,
	passthrough: Option<Box<dyn State>>,
	passthrough_factories: Vec::<Box<dyn Factory>>,
	preamble: String,
	programs: Vec<String>,
	search: String,
	selected: Option<usize>,
}

impl Default for DefaultState {
	fn default() -> Self {
		DefaultState {
			active_list: ActiveList::default(),
			autocomplete: List::default(),
			factory: Box::new(DefaultFactory),
			fuzzyfind: List::default(),
			passthrough: None,
			passthrough_factories: vec![Box::new(OpenProjectFactory)],
			preamble: String::new(),
			programs: get_programs().unwrap(),
			search: String::default(),
			selected: None,
		}
	}
}

impl State for DefaultState {
	fn get_factory(&self) -> &Box<dyn Factory> {
		&self.factory
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
		} else {
			&self.autocomplete
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
		if let Some(passthrough) = self.passthrough.as_mut() { // reset passthrough if it is no longer valid
			if !passthrough.get_factory().should_create(&search) {
				self.passthrough = None;
			}
		} else {
			for factory in self.passthrough_factories.iter() { // look for a passthrough to use
				if factory.should_create(&search) {
					self.passthrough = Some(factory.create());
					break;
				}
			}
			self.selected = Some(0);
			self.search = search.clone();
		}

		if let Some(passthrough) = self.passthrough.as_mut() {
			passthrough.update_search(search.replace(passthrough.get_preamble(), ""));
		} else {
			self.search = search;
			self.active_list = ActiveList::FuzzyFinder;

			self.autocomplete = autocomplete(&self.programs, &self.search);
			self.fuzzyfind = fuzzyfind(&self.programs, &self.search);
		}
	}

	fn autocomplete(&mut self) -> String {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return format!("{}{}", self.search, passthrough.autocomplete());
		}

		self.active_list = ActiveList::Autocomplete;
		let result = if let Some(list) = self.autocomplete.as_ref() {
			self.search = list[0].clone();
			list[0].clone()
		} else {
			self.search.clone()
		};

		self.autocomplete = autocomplete(&self.programs, &self.search);
		self.fuzzyfind = fuzzyfind(&self.programs, &self.search);

		self.selected = Some(0);

		return result;
	}

	fn get_command(&self) -> String {
		if let Some(passthrough) = self.passthrough.as_ref() {
			format!("{}{}", self.search.clone(), passthrough.get_command())
		} else {
			self.search.clone()
		}
	}

	fn select_up(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			let passthrough_string = passthrough.select_up().0;
			return (format!("{}{}", self.search, passthrough_string), Some(passthrough_string));
		}

		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if let None = self.selected {
			self.selected = Some(0);
		} else if self.selected.unwrap() != list.unwrap().len() - 1 {
			self.selected = Some(self.selected.unwrap() + 1);
		}

		if let Some(index) = self.selected {
			(list.unwrap()[index].clone(), None)
		} else {
			(String::new(), None)
		}
	}

	fn select_down(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			let passthrough_string = passthrough.select_down().0;
			return (format!("{}{}", self.search, passthrough_string), Some(passthrough_string));
		}

		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if self.selected.unwrap() != 0 {
			self.selected = Some(self.selected.unwrap() - 1);
		}

		if let Some(index) = self.selected {
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
