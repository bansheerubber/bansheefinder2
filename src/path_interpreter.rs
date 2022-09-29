use std::path::Path;

pub fn get_programs() -> Option<Vec<String>> {
	let paths = if let Ok(path) = std::env::var("PATH") {
		path
	} else {
		return None
	};

	let mut output = Vec::new();
	for directory in paths.split(':') {
		let path = Path::new(directory);
		if let Ok(read_directory) = std::fs::read_dir(path) {
			for program in read_directory { // read all programs in directory
				output.push(program.unwrap().file_name().into_string().unwrap());
			}
		}
	}

	Some(output)
}

pub fn get_projects() -> Option<Vec<String>> {
	let mut output = Vec::new();
	for program in std::fs::read_dir("/home/me/Projects").unwrap() { // read all programs in directory
		output.push(program.unwrap().file_name().into_string().unwrap());
	}

	Some(output)
}
