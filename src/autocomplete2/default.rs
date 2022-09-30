use crate::autocomplete2::types::{
	ActiveList,
	Factory,
	List,
	State,
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

#[derive(Clone, Debug, Default)]
pub struct DefaultState {
	active_list: ActiveList,
	autocomplete: List,
	fuzzyfind: List,
	programs: Vec<String>,
	search: String,
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

		return result;
	}

	fn get_command(&self) -> String {
		self.search.clone()
	}
}

#[derive(Clone, Debug)]
pub struct DefaultFactory;

impl Factory<DefaultState> for DefaultFactory {
	fn should_create(&self, search: &String) -> bool {
		if search.len() == 0 {
			true
		} else {
			false
		}
	}

	fn create(&self) -> DefaultState {
		DefaultState {
			programs: get_programs().unwrap(),
			..DefaultState::default()
		}
	}
}
