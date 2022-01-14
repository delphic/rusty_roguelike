use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]	// HACK: have thrown attack into this random_move so right now it's actually dumb AI system
#[read_component(Player)]
pub fn random_move(
	sub_world: &mut SubWorld,
	commands: &mut CommandBuffer) {
	let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
	let mut positions = <(Entity, &Point, &Health)>::query();

	movers.iter(sub_world)
		.for_each(|(entity, pos, _)| {
			let mut rng = RandomNumberGenerator::new();
			let destination = match rng.range(0, 4) {
				0 => Point::new(-1, 0),
				1 => Point::new(1, 0),
				2 => Point::new(0, -1),
				_ => Point::new(0, 1),
			} + *pos;
			
			// This would be better if we got valid locations and selected from that vec
			// Although we'd have to readd the map resource
			// Should check existing selected destinations as well as map points
			// Should also *prefer* to attack rather than just checking destination, but this is dumb AI atm

			let mut should_move = true;
			positions.iter(sub_world)
				.filter(|(_, target_pos, _)| **target_pos == destination)
				.for_each(|(victim, _, _)| {
					if sub_world.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
						commands.push(((), WantsToAttack { 
							attacker: *entity,
							victim:  *victim
						}));
					}
					should_move = false; // Don't attack friendlies or move into their space (this is actually their current position not destination so this doesn't actually work but again poc)
				}
			);

			if should_move {
				commands.push(((), WantsToMove { entity: *entity, destination }));
			}
		})
}