use agrajag::*;

fn main() {
	let game = game::Game::new(vec![
		Room {
			opening: Command::new("look"),
			items: vec![
				Item {
					names: vec!["gun".to_string(), "blaster".to_string()],
					events: vec![
						Event {
							command_type: CommandType::Look,
							event_type: EventType::Print {
								text: "It's a gun, \
								what more could a gal ask for?".to_string()
							}
						},
						Event {
							command_type: CommandType::Custom(vec!["yes".to_string(), "no".to_string()]),
							event_type: EventType::Print {
								text: "Maybe gun.".to_string()
							}
						}
					]
				}
			],
			events: vec![
				Event {
					command_type: CommandType::Look,
					event_type: EventType::Print {
						text: "It's a room, \
						what more could a gal ask for?".to_string()
					}
				}
			]
		}
	]);

	game.run();
}
