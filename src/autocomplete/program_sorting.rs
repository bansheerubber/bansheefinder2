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
	let mut common_start = String::new();
	for program in programs.iter() {
		if let Some(index) = program.find(search) {
			if index == 0 && program.len() > common_start.len() {
				common_start = program.clone()
			}
		}
	}

	let mut output = programs.iter()
		.fold(Vec::new(), |mut acc, program| {
			if let Some(index) = program.find(search) {
				if index == 0 {
					acc.push(program.clone());

					let mut stop = None;
					let mut program_chars = program.chars();
					let mut common_start_chars = common_start.chars();
					for i in 0..std::cmp::min(program.len(), common_start.len()) {
						if program_chars.next() != common_start_chars.next() {
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
			sort_program(a, b, program_frequency)
		}
	);

	Some(Autocomplete {
		common_start,
		list: Some(output),
	})
}
