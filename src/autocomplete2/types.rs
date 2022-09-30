pub type List = Option<Vec<String>>;

#[derive(Clone, Copy, Debug, Default)]
pub enum ActiveList {
	Autocomplete,
	#[default]
	FuzzyFinder,
}

pub fn get_ui_list<'a>(state: &'a ActiveList, autocomplete: &'a List, fuzzyfind: &'a List) -> &'a List {
	match state {
		ActiveList::Autocomplete => autocomplete,
		ActiveList::FuzzyFinder => fuzzyfind,
	}
}

pub trait State {
	fn get_active_list(&self) -> ActiveList;
	fn get_autocomplete_list(&self) -> &List;
	fn get_fuzzyfinder_list(&self) -> &List;

	fn update_search(&mut self, search: String);
	fn autocomplete(&mut self) -> String;
	fn get_command(&self) -> String;

	fn select_up(&mut self) -> String;
	fn select_down(&mut self) -> String;

	// the list that we'll use in the ui
	fn get_ui_list(&self) -> &List {
		match self.get_active_list() {
			ActiveList::Autocomplete => self.get_autocomplete_list(),
			ActiveList::FuzzyFinder => self.get_fuzzyfinder_list(),
		}
	}
}

pub trait Factory<T: State> {
	fn should_create(search: &String) -> bool;
	fn create() -> T;
}
