pub fn file_fuzzyfind(programs: &Vec<String>, search: &String) -> Option<Vec<String>> {
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

pub fn file_autocomplete(programs: &Vec<String>, search: &String) -> Option<Vec<String>> {
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

pub fn project_fuzzyfind(projects: &Vec<String>, search: &String) -> Option<Vec<String>> {
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

pub fn project_autocomplete(projects: &Vec<String>, search: &String) -> Option<Vec<String>> {
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
