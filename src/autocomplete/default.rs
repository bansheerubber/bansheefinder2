use crate::autocomplete::open_project::OpenProjectFactory;
use crate::autocomplete::sudo::SudoFactory;
use crate::autocomplete::types::{
	ActiveList,
	CommandType,
	Factory,
	List,
	State,
	get_ui_list,
	handle_update_placeholder,
	passthrough_command,
	passthrough_string,
};
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
			passthrough_factories: vec![Box::new(OpenProjectFactory), Box::new(SudoFactory)],
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
		if handle_update_placeholder(&search, &mut self.passthrough, &self.passthrough_factories) {
			let mut search = search;
			search.drain(0..self.passthrough.as_ref().unwrap().get_preamble().len());

			self.selected = Some(0);
			self.search = self.passthrough.as_ref().unwrap().get_preamble().clone();

			self.passthrough.as_mut().unwrap().update_search(search);
		} else {
			self.search = search;
			self.active_list = ActiveList::FuzzyFinder;

			self.autocomplete = autocomplete(&self.programs, &self.search);
			self.fuzzyfind = fuzzyfind(&self.programs, &self.search);
		}
	}

	fn autocomplete(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.autocomplete());
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

		(result, None)
	}

	fn get_command(&self) -> (String, CommandType) {
		passthrough_command(&self.search, CommandType::Normal, &self.passthrough)
	}

	fn select_up(&mut self) -> (String, Option<String>) {
		if let Some(passthrough) = self.passthrough.as_mut() {
			return passthrough_string(&self.search, passthrough.select_up());
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
			return passthrough_string(&self.search, passthrough.select_down());
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
