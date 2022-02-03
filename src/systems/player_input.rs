use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[write_component(Health)]
#[read_component(Weapon)]
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
			VirtualKeyCode::Key1 => use_item(0, sub_world, commands),
			VirtualKeyCode::Key2 => use_item(1, sub_world, commands),
			VirtualKeyCode::Key3 => use_item(2, sub_world, commands),
			VirtualKeyCode::Key4 => use_item(3, sub_world, commands),
			VirtualKeyCode::Key5 => use_item(4, sub_world, commands),
			VirtualKeyCode::Key6 => use_item(5, sub_world, commands),
			VirtualKeyCode::Key7 => use_item(6, sub_world, commands),
			VirtualKeyCode::Key8 => use_item(7, sub_world, commands),
			VirtualKeyCode::Key9 => use_item(8, sub_world, commands),
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
		}

		*turn_state = TurnState::PlayerTurn;
	}
}

fn use_item(n: usize, sub_world: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
	let player_entity = <(Entity, &Player)>::query()
		.iter(sub_world)
		.find_map(|(entity, _player)| Some(*entity))
		.unwrap();
	
	let item_entity = <(Entity, &Item, &Carried)>::query()
		.iter(sub_world)
		.filter(|(entity, _, carried)| { 
			let is_weapon = if let Ok(e) = sub_world.entry_ref(**entity) {
				e.get_component::<Weapon>().is_ok()
			} else {
				false
			};
			carried.0 == player_entity && !is_weapon
		})
		.enumerate()
		.filter(|(item_count, (_, _, _))| *item_count == n)
		.find_map(|(_, (item_entity, _, _))| Some(*item_entity));
	
	if let Some(item_entity) = item_entity {
		commands.push(((), ActivateItem { used_by: player_entity, item: item_entity }));
	}
	Point::zero()
}