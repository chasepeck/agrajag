use crate::*;

pub struct Room {
	pub opening: Command,
	pub items: Vec<Item>,
	pub events: Vec<Event>
}

impl Room {
	fn item(&self, name: &str) -> Option<&Item> {
		for i in &self.items {
			if i.names.contains(&name.to_string()) { return Some(i); }
		}
		None
	}

	fn item_param(&self, param: &Vec<String>) -> Option<&Item> {
		for i in param {
			let item = self.item(i);
			if let Some(_) = item { return item; }
		}
		None
	}

	pub fn event(&self, command: &Command) {
		if let Some(i) = self.item_param(&command.param) {
			for i in &i.events {
				if i.match_command(command) { i.exec(); return; }
			}
		} else {
			for i in &self.events {
				if i.match_command(command) { i.exec(); return; }
			}
		}

		match &command.command_type {
			CommandType::Custom(commands) => {
				println!("I don't know how to {}.", commands[0]);
			},
			_ => {
				if &command.param.len() > &0 {
					println!("I can't {} on that.", &command.command_strings()[0]);
				} else {
					println!("I can't {}.", &command.command_strings()[0]);
				}
			}
		}
	}
}
