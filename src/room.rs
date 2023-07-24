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
				if i.match_command(command) { i.exec(); }
			}
		} else {
			for i in &self.events {
				if i.match_command(command) { i.exec(); }
			}
		}
	}
}
