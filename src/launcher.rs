use std::time::{ SystemTime, UNIX_EPOCH };

use crate::path_interpreter:: {
	ProgramFrequency,
	read_command_frequency,
	write_command_frequency,
};

pub fn launch_program(program: String) {
	let mut map = read_command_frequency();
	let default = ProgramFrequency::default();
	let program_frequency = map.get(&program).unwrap_or(&default);
	map.insert(
		program.clone(),
		ProgramFrequency {
			count: program_frequency.count + 1,
			timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
		}
	);
	write_command_frequency(map);

	let result = std::process::Command::new("sh")
		.arg("-c")
		.arg(format!("i3-msg exec {}", program))
		.output();

	if let Err(error) = result {
		eprintln!("could not launch {:?}", error);
	}

	std::process::exit(1);
}

pub fn launch_program_sudo(program: String, password: String) {
	let program = program[5..].to_string(); // remove sudo preamble

	let result = std::process::Command::new("sh")
		.arg("-c")
		.arg(format!("echo \"{}\" | sudo -S {}", password, program)) // TODO better password piping
		.spawn();

	if let Err(error) = result {
		eprintln!("could not launch {:?}", error);
	}

	std::process::exit(1);
}
