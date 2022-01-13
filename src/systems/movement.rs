use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
	entity: &Entity,
	move_intent: &WantsToMove,
	#[resource] map: &Map,
	#[resource] camera: &mut Camera,
	sub_world: &mut SubWorld,
	commands: &mut CommandBuffer
	) {
		if map.can_enter_tile(move_intent.destination) {
			// Add a new point component -> i.e. move the entity
			commands.add_component(move_intent.entity, move_intent.destination);

			if sub_world.entry_ref(move_intent.entity)
				.unwrap()
				.get_component::<Player>().is_ok()
			{
				camera.set_position(move_intent.destination);
			}
		}
		commands.remove(*entity);
	}