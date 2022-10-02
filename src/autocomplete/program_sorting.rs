use std::cmp::Ordering;

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
) -> Option<Vec<String>> {
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
			sort_program(a, b, program_frequency)
		}
	);

	Some(output)
}
