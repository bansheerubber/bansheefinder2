use std::cmp::Ordering;

use crate::autocomplete::types::Autocomplete;
use crate::path_interpreter::{
	ProgramFrequencyMap,
	compare_program_frequency,
};

fn sort_program(a: &String, b: &String, program_frequency: &ProgramFrequencyMap) -> Ordering {
	if program_frequency.map.contains_key(a) && program_frequency.map.contains_key(b) {
		compare_program_frequency(&program_frequency.map[b], &program_frequency.map[a], program_frequency)
	} else if program_frequency.map.contains_key(a) && !program_frequency.map.contains_key(b) {
		Ordering::Less
	} else if !program_frequency.map.contains_key(a) && program_frequency.map.contains_key(b) {
		Ordering::Greater
	} else {
		a.len().cmp(&b.len())
	}
}

pub fn fuzzyfind(
	programs: &Vec<String>,
	program_frequency: &ProgramFrequencyMap,
	search: &String
) -> Option<Vec<String>> {
	let mut output = programs.iter()
		.fold(Vec::new(), |mut acc, program| {
			if program.find(search).is_some() {
				acc.push(program.clone())
			}

			acc
		});

	output.sort_by(
		|a, b| {
			sort_program(a, b, program_frequency)
		}
	);

	Some(output)
}

pub fn autocomplete(
	programs: &Vec<String>,
	program_frequency: &ProgramFrequencyMap,
	search: &String
) -> Option<Autocomplete> {
	let mut common_start: Option<String> = None;
	let mut output = programs.iter()
		.fold(Vec::new(), |mut acc, program| {
			if let Some(index) = program.find(search) {
				if index == 0 {
					acc.push(program.clone());

					if let Some(common) = common_start.as_ref() {
						let mut stop = None;
						let mut program_chars = program.chars();
						let mut common_start_chars = common.chars();
						for i in 0..std::cmp::min(program.len(), common.len()) {
							if program_chars.next() != common_start_chars.next() {
								stop = Some(i);
								break;
							}
						}

						if stop == None && common.len() > program.len() {
							stop = Some(program.len());
						}

						if let Some(index) = stop {
							common_start = Some(common[0..index].to_string());
						}
					} else {
						common_start = Some(program.clone());
					}
				}
			}

			acc
		});

	output.sort_by(
		|a, b| {
			sort_program(a, b, program_frequency)
		}
	);

	Some(Autocomplete {
		common_start: common_start.unwrap_or_default(),
		list: Some(output),
	})
}
