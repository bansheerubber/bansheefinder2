pub fn launch_program(program: String) {
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
