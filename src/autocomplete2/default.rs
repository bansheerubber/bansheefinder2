use crate::autocomplete2::types::{
	ActiveList,
	Factory,
	List,
	State,
	get_ui_list,
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
	fuzzyfind: List,
	passthrough_factories: Vec::<Box<dyn Factory>>,
	programs: Vec<String>,
	search: String,
	selected: Option<usize>,
}

impl Default for DefaultState {
	fn default() -> Self {
		DefaultState {
			active_list: ActiveList::default(),
			autocomplete: List::default(),
			fuzzyfind: List::default(),
			passthrough_factories: Vec::default(),
			programs: get_programs().unwrap(),
			search: String::default(),
			selected: None,
		}
	}
}

impl State for DefaultState {
	fn get_active_list(&self) -> ActiveList {
		self.active_list
	}

	fn get_autocomplete_list(&self) -> &List {
		&self.autocomplete
	}

	fn get_fuzzyfinder_list(&self) -> &List {
		&self.fuzzyfind
	}

	fn update_search(&mut self, search: String) {
		self.search = search;
		self.active_list = ActiveList::FuzzyFinder;

		self.autocomplete = autocomplete(&self.programs, &self.search);
		self.fuzzyfind = fuzzyfind(&self.programs, &self.search);
	}

	fn autocomplete(&mut self) -> String {
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
		self.search.clone()
	}

	fn select_up(&mut self) -> String {
		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if let None = self.selected {
			self.selected = Some(0);
		} else if self.selected.unwrap() != list.unwrap().len() - 1 {
			self.selected = Some(self.selected.unwrap() + 1);
		}

		if let Some(index) = self.selected {
			list.unwrap()[index].clone()
		} else {
			String::new()
		}
	}

	fn select_down(&mut self) -> String {
		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if self.selected.unwrap() != 0 {
			self.selected = Some(self.selected.unwrap() - 1);
		}

		if let Some(index) = self.selected {
			list.unwrap()[index].clone()
		} else {
			String::new()
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
