use crate::autocomplete2::types::{
	ActiveList,
	List,
	State,
};

#[derive(Clone, Debug)]
pub struct DefaultState {
	active_list: ActiveList,
	autocomplete: List,
	fuzzyfinder: List,
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
		&self.fuzzyfinder
	}

	fn update_search(&mut self, search: String) {
		self.search = search;
		self.active_list = ActiveList::FuzzyFinder;
	}

	fn autocomplete(&mut self) -> String {
		self.active_list = ActiveList::Autocomplete;
		if let Some(autocomplete) = self.autocomplete.as_ref() {
			self.search = autocomplete[0].clone();
			autocomplete[0].clone()
		} else {
			String::new()
		}
	}

	fn get_command(&self) -> String {
		self.search.clone()
	}
}
