use std::cmp::Ordering;
use std::collections::HashMap;

use crate::path_interpreter::ProgramFrequency;

fn sort_program(a: &String, b: &String, program_frequency: &HashMap<String, ProgramFrequency>) -> Ordering {
	if program_frequency.contains_key(a) && program_frequency.contains_key(b) {
		program_frequency[a].cmp(&program_frequency[b])
	} else if program_frequency.contains_key(a) && !program_frequency.contains_key(b) {
		Ordering::Less
	} else if !program_frequency.contains_key(a) && program_frequency.contains_key(b) {
		Ordering::Greater
	} else {
		a.len().cmp(&b.len())
	}
}

pub fn fuzzyfind(
	programs: &Vec<String>,
	program_frequency: &HashMap<String, ProgramFrequency>,
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
	program_frequency: &HashMap<String, ProgramFrequency>,
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
