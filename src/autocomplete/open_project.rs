use crate::autocomplete::types::{
	ActiveList,
	Autocomplete,
	CommandType,
	Factory,
	List,
	State,
	get_ui_list,
};
use crate::path_interpreter::get_projects;

fn fuzzyfind(projects: &Vec<String>, search: &String) -> List {
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

fn autocomplete(projects: &Vec<String>, search: &String) -> Option<Autocomplete> {
	let mut common_start = String::new();
	for project in projects.iter() {
		if let Some(index) = project.find(search) {
			if index == 0 && project.len() > common_start.len() {
				common_start = project.clone()
			}
		}
	}

	let mut output = projects.iter()
		.fold(Vec::new(), |mut acc, project| {
			if let Some(index) = project.find(search) {
				if index == 0 {
					acc.push(project.clone());

					let mut stop = None;
					let mut project_chars = project.chars();
					let mut common_start_chars = common_start.chars();
					for i in 0..std::cmp::min(project.len(), common_start.len()) {
						if project_chars.next() != common_start_chars.next() {
							stop = Some(i);
						}
					}

					if let Some(index) = stop {
						common_start = common_start[0..index].to_string();
					}
				}
			}

			acc
		});

	output.sort_by(
		|a, b| {
			a.len().cmp(&b.len())
		}
	);

	Some(Autocomplete {
		common_start,
		list: Some(output),
	})
}

pub struct OpenProjectState {
	active_list: ActiveList,
	autocomplete: Option<Autocomplete>,
	factory: Box<dyn Factory>,
	fuzzyfind: List,
	preamble: String,
	projects: Vec<String>,
	search: String,
	selected: Option<usize>,
}

impl Default for OpenProjectState {
	fn default() -> Self {
		OpenProjectState {
			active_list: ActiveList::default(),
			autocomplete: None,
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
		if let Some(autocomplete) = self.autocomplete.as_ref() {
			&autocomplete.list
		} else {
			&None
		}
	}

	fn get_fuzzyfinder_list(&self) -> &List {
		&self.fuzzyfind
	}

	fn update_search(&mut self, search: String) {
		self.search = search;
		self.active_list = ActiveList::FuzzyFinder;
		self.selected = None;

		self.autocomplete = autocomplete(&self.projects, &self.search);
		self.fuzzyfind = fuzzyfind(&self.projects, &self.search);
	}

	fn autocomplete(&mut self) -> (String, Option<String>) {
		self.active_list = ActiveList::Autocomplete;
		if let Some(list) = self.autocomplete.as_ref() {
			self.search = list.common_start.clone();
		}

		self.autocomplete = autocomplete(&self.projects, &self.search);
		self.fuzzyfind = fuzzyfind(&self.projects, &self.search);

		self.selected = Some(0);

		(self.search.clone(), None)
	}

	fn get_command(&self) -> (String, Option<String>, CommandType) { // only returns the project folder name
		(self.search.clone(), None, CommandType::OpenProject)
	}

	fn select_up(&mut self) -> (String, Option<String>) {
		let list = get_ui_list(&self.active_list, &self.autocomplete, &self.fuzzyfind).as_ref();
		if let None = list {
			self.selected = None;
		} else if self.selected.is_some() && self.selected.unwrap() != list.unwrap().len() - 1 {
			self.selected = Some(self.selected.unwrap() + 1);
		} else {
			self.selected = Some(0);
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
		} else if self.selected.is_some() && self.selected.unwrap() != 0 {
			self.selected = Some(self.selected.unwrap() - 1);
		} else {
			self.selected = Some(list.unwrap().len() - 1);
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
