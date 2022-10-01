pub type List = Option<Vec<String>>;

pub enum CommandType {
	Normal,
	OpenProject,
	Sudo,
}

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
	fn get_factory(&self) -> &Box<dyn Factory>;

	fn get_preamble(&self) -> &String;

	fn get_active_list(&self) -> ActiveList;
	fn get_autocomplete_list(&self) -> &List;
	fn get_fuzzyfinder_list(&self) -> &List;

	fn update_search(&mut self, search: String);
	fn autocomplete(&mut self) -> (String, Option<String>);
	fn get_command(&self) -> (String, CommandType);

	fn select_up(&mut self) -> (String, Option<String>);
	fn select_down(&mut self) -> (String, Option<String>);

	// the list that we'll use in the ui
	fn get_ui_list(&self) -> &List {
		match self.get_active_list() {
			ActiveList::Autocomplete => self.get_autocomplete_list(),
			ActiveList::FuzzyFinder => self.get_fuzzyfinder_list(),
		}
	}
}

pub trait Factory {
	fn should_create(&self, search: &String) -> bool;
	fn create(&self) -> Box<dyn State>;
}

pub fn handle_update_placeholder(
	search: &String,
	passthrough: &mut Option<Box<dyn State>>,
	passthrough_factories: &Vec<Box<dyn Factory>>
) -> bool {
	if let Some(pt) = passthrough { // reset passthrough if it is no longer valid
		if !pt.get_factory().should_create(&search) {
			*passthrough = None;
		}
	} else {
		for factory in passthrough_factories.iter() { // look for a passthrough to use
			if factory.should_create(&search) {
				*passthrough = Some(factory.create());
				break;
			}
		}
	}

	return passthrough.is_some();
}

pub fn passthrough_string(
	search: &String,
	passthrough: (String, Option<String>)
) -> (String, Option<String>) {
	if passthrough.1.is_none() {
		(format!("{}{}", search, passthrough.0), Some(passthrough.0))
	} else {
		(format!("{}{}", search, passthrough.0), passthrough.1)
	}
}

pub fn passthrough_command(
	search: &String,
	command_type: CommandType,
	passthrough: &Option<Box<dyn State>>
) -> (String, CommandType) {
	if let Some(passthrough) = passthrough.as_ref() {
		let (command, command_type) = passthrough.get_command();
		(format!("{}{}", search, command), command_type)
	} else {
		(search.clone(), command_type)
	}
}
