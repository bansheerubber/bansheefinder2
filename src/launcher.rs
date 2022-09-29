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
