use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
	sub_world: &mut SubWorld,
	commands: &mut CommandBuffer) {
	let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
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

			commands.push(((), WantsToMove { entity: *entity, destination }));
		})
}