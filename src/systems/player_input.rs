use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
	sub_world: &mut SubWorld,
	#[resource] key: &Option<VirtualKeyCode>,
	#[resource] turn_state: &mut TurnState,
	commands: &mut CommandBuffer
	) {
		if let Some(key) = key {
			let delta = match key {
				VirtualKeyCode::Left => Point::new(-1, 0),
				VirtualKeyCode::Right => Point::new(1, 0),
				VirtualKeyCode::Up => Point::new(0, -1),
				VirtualKeyCode::Down => Point::new(0, 1),
				_ => Point::new(0, 0),
			};

			// Don't check for delta != 0 as want to be able to skip turn
			let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
			players.iter(sub_world).for_each(|(entity, pos)| {
				let destination = *pos + delta;
				commands.push(( (), WantsToMove{ entity: *entity, destination } ));
			});
			*turn_state = TurnState::PlayerTurn;
		}
}