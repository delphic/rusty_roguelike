use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Item)]
#[read_component(Point)]
#[read_component(Carried)]
pub fn movement(
	entity: &Entity,
	move_intent: &WantsToMove,
	#[resource] map: &mut Map,
	#[resource] camera: &mut Camera,
	sub_world: &mut SubWorld,
	commands: &mut CommandBuffer
	) {
		if map.can_enter_tile(move_intent.destination) {
			// Add a new point component -> i.e. move the entity
			commands.add_component(move_intent.entity, move_intent.destination);

			if let Ok(entry) = sub_world.entry_ref(move_intent.entity) {
				if let Ok(fov) = entry.get_component::<FieldOfView>() {
					commands.add_component(move_intent.entity, fov.clone_dirty());
					// ^^ Add and replace existing fov - which marks it dirty

					if entry.get_component::<Player>().is_ok() {
						camera.set_position(move_intent.destination);
						fov.visible_tiles.iter().for_each(|pos| {
							map.revealed_tiles[get_map_idx(pos.x, pos.y)] = true;
						});

						// Arguably: could add itent to pick up and system for that instead of muddy in the movement
						// Pickup items
						let mut items = <(Entity, &Item, &Point)>::query();
						items.iter(sub_world)
							.filter(|(_entity, _item, &item_pos)| item_pos == move_intent.destination)
							.for_each(|(entity, _item, _item_pos)| {
								commands.remove_component::<Point>(*entity);
								commands.add_component(*entity,Carried(move_intent.entity));
						});	
					}
				}
			}
		}
		commands.remove(*entity);
	}