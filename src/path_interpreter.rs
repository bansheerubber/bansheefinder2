use chrono::Local;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{ File, OpenOptions };
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Default, Eq, PartialEq)]
pub struct ProgramFrequency {
	pub count: u16,
	pub timestamp: u64,
}

#[derive(Default)]
pub struct ProgramFrequencyMap {
	pub map: HashMap<String, ProgramFrequency>,
	pub max: u16,
}

pub fn compare_program_frequency(
	this: &ProgramFrequency,
	other: &ProgramFrequency,
	frequencies: &ProgramFrequencyMap
) -> Ordering {
	let this_count_normalized = this.count as f64 / frequencies.max as f64;
	let other_count_normalized = other.count as f64 / frequencies.max as f64;

	let timestamp = Local::now().timestamp() as u64;

	let this_duration_normalized = (1.07 as f64).powf((timestamp - this.timestamp) as f64 / (60.0 * 60.0 * 24.0));
	let other_duration_normalized = (1.07 as f64).powf((timestamp - other.timestamp) as f64 / (60.0 * 60.0 * 24.0));

	let this_score = this_count_normalized / 2.0 + this_duration_normalized / 2.0;
	let other_score = other_count_normalized / 2.0 + other_duration_normalized / 2.0;

	if this_score < other_score {
		Ordering::Less
	} else if this_score > other_score {
		Ordering::Greater
	} else {
		Ordering::Equal
	}
}

impl Ord for ProgramFrequency {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.count < other.count {
			Ordering::Less
		} else if self.count > other.count {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl PartialOrd for ProgramFrequency {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn read_command_frequency() -> ProgramFrequencyMap {
	let file = OpenOptions::new().read(true).open("/home/me/.local/share/bansheefinder/frequency.map");
	let mut file = if let Err(_) = file {
		return ProgramFrequencyMap::default();
	} else {
		file.unwrap()
	};

	let mut contents = Vec::<u8>::new();
	if let Err(error) = file.read_to_end(&mut contents) {
		eprintln!("Could not read file {:?}", error);
		std::process::exit(0); // terminate program right away so we don't overwrite data
	}

	let mut map = HashMap::new();
	let mut max = 0;
	let mut index = 0;
	while index < contents.len() {
		let string_size = contents[index];
		index += 1;

		let string = String::from_utf8(contents[index..index + string_size as usize].to_vec()).unwrap();
		index += string_size as usize;

		let count = (contents[index + 1] as u16) << 8 | contents[index] as u16;
		index += 2;

		if count > max {
			max = count;
		}

		let mut timestamp = 0;
		for i in 0..8 {
			timestamp |= (contents[index] as u64) << (i * 8);
			index += 1;
		}

		map.insert(
			string,
			ProgramFrequency {
				count,
				timestamp,
			}
		);
	}

	return ProgramFrequencyMap {
		max,
		map,
	};
}

fn handle_write(file: &mut File, buffer: &[u8]) -> bool {
	if let Err(error) = file.write(buffer) {
		eprintln!("Could not write to file {:?})", error);
		return false;
	} else {
		return true;
	}
}

pub fn write_command_frequency(map: ProgramFrequencyMap) {
	let file = OpenOptions::new().write(true).create(true).open("/home/me/.local/share/bansheefinder/frequency.map");
	let mut file = if let Err(error) = file {
		eprintln!("Could not write file {:?}", error);
		return;
	} else {
		file.unwrap()
	};

	for (key, value) in map.map {
		if key.len() > 255 { // TODO fix this from happening in a nice way
			continue;
		}

		if !handle_write(&mut file, &[key.len() as u8])
			|| !handle_write(&mut file, key.as_bytes())
			|| !handle_write(&mut file, &[(value.count & 0xFF) as u8])
			|| !handle_write(&mut file, &[((value.count & 0xFF00) >> 8) as u8])
		{
			return;
		}

		// write timestamp
		for i in 0..8 {
			if !handle_write(&mut file, &[(value.timestamp >> (i * 8)) as u8 & 0xFF]) {
				return;
			}
		}
	}
}

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
