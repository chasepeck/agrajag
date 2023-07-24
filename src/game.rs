use crate::*;
use std::io::{self, Write};

pub struct Game {
	pub mode: Mode,
	pub position: usize,
	pub rooms: Vec<Room>
}

impl Game {
	pub fn new(rooms: Vec<Room>) -> Game {
		Game {
			mode: Mode::Do,
			position: 0,
			rooms
		}
	}

	pub fn run(&self) {
		self.rooms[0].opening.exec(&self);
		loop {
			self.update();
		}
	}

	fn update(&self) {
		let mut input = "".to_string();
		print!("\n| {} > ", self.mode());
		io::stdout().flush().unwrap();
	 
		io::stdin().read_line(&mut input).expect("Failed to read input");
		println!("");
		
		Command::new(&input).exec(&self);
	}

	fn mode(&self) -> String {
		match &self.mode {
			Mode::Do => "DO".to_string(),
			Mode::Say => "SAY".to_string(),
			Mode::Custom(mode) => mode.to_uppercase().to_string()
		}
	}

	pub fn set_mode(&mut self, mode: Mode) {
		self.mode = mode;
	}
}

pub enum Mode {
	Do,
	Say,
	Custom(String)
}
