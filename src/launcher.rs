use chrono::Local;

use crate::path_interpreter:: {
	ProgramFrequency,
	read_command_frequency,
	write_command_frequency,
};

pub fn update_frequency(program: &String) {
	let mut frequency = read_command_frequency();
	let default = ProgramFrequency::default();
	let program_frequency = frequency.map.get(program).unwrap_or(&default);
	frequency.map.insert(
		program.clone(),
		ProgramFrequency {
			count: program_frequency.count + 1,
			timestamp: Local::now().timestamp() as u64,
		}
	);
	write_command_frequency(frequency);
}

pub fn launch_program(program: String, base_command: Option<String>) {
	if program.len() == 0 {
		return;
	}

	if let Some(base_command) = base_command {
		update_frequency(&base_command);
	}

	let result = std::process::Command::new("sh")
		.arg("-c")
		.arg(format!("i3-msg exec {}", program))
		.output();

	if let Err(error) = result {
		eprintln!("could not launch {:?}", error);
	}

	std::process::exit(1);
}

pub fn launch_program_sudo(program: String, base_command: Option<String>, password: String) {
	if program.len() == 0 {
		return;
	}

	if let Some(base_command) = base_command {
		update_frequency(&base_command);
	}

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
