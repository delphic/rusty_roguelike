use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
	sub_world: &mut SubWorld,
	#[resource] map: &Map,
	#[resource] key: &Option<VirtualKeyCode>,
	#[resource] camera: &mut Camera,
	#[resource] turn_state: &mut TurnState
	) {
		if let Some(key) = key {
			let delta = match key {
				VirtualKeyCode::Left => Point::new(-1, 0),
				VirtualKeyCode::Right => Point::new(1, 0),
				VirtualKeyCode::Up => Point::new(0, -1),
				VirtualKeyCode::Down => Point::new(0, 1),
				_ => Point::new(0, 0),
			};

			if delta.x != 0 || delta.y != 0 {
				let mut players = <&mut Point>::query().filter(component::<Player>());
				// Why don't we just do <(Point, Player)>::query() ? 'cause we want to do iter_mut and we don't want write access to Player?
				players.iter_mut(sub_world).for_each(|pos| {
					let destination = *pos + delta;
					if map.can_enter_tile(destination) {
						*pos = destination;
						camera.set_position(destination);
						*turn_state = TurnState::PlayerTurn;
					}
				});
			}
		}
}