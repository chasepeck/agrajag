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
							event_type: EventType::Print(
								"It's a gun, \
								what more could a gal ask for?".to_string()
							)
						},
						Event {
							command_type: CommandType::Custom(vec!["yes".to_string(), "no".to_string()]),
							event_type: EventType::Print("Maybe gun.".to_string())
						}
					]
				}
			],
			events: vec![
				Event {
					command_type: CommandType::Look,
					event_type: EventType::Multi(
						vec![
							EventType::Print("Ok!".to_string()),
							EventType::Print("No!".to_string())
						]
					)
				}
			]
		}
	]);

	game.run();
}
