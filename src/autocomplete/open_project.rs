use crate::autocomplete::types::{
	ActiveList,
	Factory,
	List,
	State,
	get_ui_list,
};
use crate::path_interpreter::get_projects;

fn fuzzyfind(projects: &Vec<String>, search: &String) -> Option<Vec<String>> {
	let mut output = projects.iter()
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

fn autocomplete(projects: &Vec<String>, search: &String) -> Option<Vec<String>> {
	let mut output = projects.iter()
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

pub struct OpenProjectState {
	active_list: ActiveList,
	autocomplete: List,
	factory: Box<dyn Factory>,
	fuzzyfind: List,
	preamble: String,
	projects: Vec<String>,
	search: String,
	selected: Option<usize>,
}

impl Default for OpenProjectState {
	fn default() -> Self {
		println!("hey there");
		OpenProjectState {
			active_list: ActiveList::default(),
			autocomplete: List::default(),
			factory: Box::new(OpenProjectFactory),
			fuzzyfind: List::default(),
			preamble: String::from("open-project "),
			projects: get_projects().unwrap(),
			search: String::default(),
			selected: None,
		}
	}
}

impl State for OpenProjectState {
	fn get_factory(&self) -> &Box<dyn Factory> {
		&self.factory
	}

	fn get_preamble(&self) -> &String {
		&self.preamble
	}

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

		self.autocomplete = autocomplete(&self.projects, &self.search);
		self.fuzzyfind = fuzzyfind(&self.projects, &self.search);
	}

	fn autocomplete(&mut self) -> String {
		self.active_list = ActiveList::Autocomplete;
		let result = if let Some(list) = self.autocomplete.as_ref() {
			self.search = list[0].clone();
			list[0].clone()
		} else {
			self.search.clone()
		};
		self.autocomplete = autocomplete(&self.projects, &self.search);
		self.fuzzyfind = fuzzyfind(&self.projects, &self.search);

		self.selected = Some(0);

		return result;
	}

	fn get_command(&self) -> String { // only returns the project folder name
		self.search.clone()
	}

	fn select_up(&mut self) -> (String, Option<String>) {
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
pub struct OpenProjectFactory;

impl Factory for OpenProjectFactory {
	fn should_create(&self, search: &String) -> bool {
		if search.len() >= 13 && &search[0..12] == "open-project" {
			true
		} else {
			false
		}
	}

	fn create(&self) -> Box<dyn State> {
		Box::new(OpenProjectState::default())
	}
}
