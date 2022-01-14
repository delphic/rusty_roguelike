use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)] // Don't actually think we need this?
#[write_component(Health)]
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

			let (player_entity, destination) = players
				.iter(sub_world)
				.find_map(|(entity, pos)| Some((*entity, *pos + delta)))
				.unwrap();

			
			let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
			if delta.x != 0 || delta.y != 0 {
				let mut is_attacking = false;
				enemies
					.iter(sub_world)
					.filter(|(_, pos)| { **pos == destination })
					.for_each(|(entity, _)| {
						is_attacking = true;
						commands.push(((), WantsToAttack { attacker: player_entity, victim: *entity }));
					});
				
				if !is_attacking {
					commands.push(((), WantsToMove { entity: player_entity, destination }));
				}
			} else {
				if let Ok(mut health) = sub_world.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
					health.current = i32::min(health.max, health.current + 1);
				}
			}

			*turn_state = TurnState::PlayerTurn;
		}
}