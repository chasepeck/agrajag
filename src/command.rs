use crate::*;

pub struct Command {
	pub command_type: CommandType,
	pub param: Vec<String>
}

impl Command {
	pub fn new(input: &str) -> Command {
		let input_trim = input.trim().to_lowercase();
		let mut iter = input_trim.split_whitespace();
		let command = iter.next().unwrap_or_default().to_string();
		let param: Vec<String> = iter.map(|x| x.to_string()).collect();

		let command_type = {
			if vec!["look".to_string()].contains(&command) {
				CommandType::Look
			} else {
				CommandType::Custom(vec![command])
			}
		};

		Command {
			command_type,
			param
		}
	}

	pub fn exec(&self, game: &Game) {
		match &self.command_type {
			// TODO: Quit, Inventory (global commands)
			_ => { game.rooms[game.position].event(self); }
		}
	}

	pub fn command_strings(&self) -> Vec<String> {
		self.command_type.command_strings()
	}
}

#[derive(PartialEq)]
pub enum CommandType {
	Look,
	Custom(Vec<String>)
}

impl CommandType {
	pub fn command_strings(&self) -> Vec<String> {
		match &self {
			CommandType::Look => vec![
				"look".to_string(),
			],
			CommandType::Custom(commands) => commands.to_vec()
		}
	}
}

pub struct Event {
	pub command_type: CommandType,
	pub event_type: EventType
}

impl Event {
	pub fn match_command(&self, command: &Command) -> bool {
		if let CommandType::Custom(i) = &command.command_type {
			if let CommandType::Custom(ii) = &self.command_type {
				return ii.contains(&i[0]);
			}
		}
		self.command_type == command.command_type
	}

	pub fn exec(&self) {
		let _ = &self.event_type.exec();
	}
}

pub enum EventType {
	Print(String),
	Multi(Vec<EventType>),
	Custom { commands: Vec<String> }
}

impl EventType {
	pub fn exec(&self) {
		match &self {
			EventType::Print(text) => { println!("{}", text); },
			EventType::Multi(events) => { for i in events { i.exec(); } },
			_ => ()
		}
	}
}
