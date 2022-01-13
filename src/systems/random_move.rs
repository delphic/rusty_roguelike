use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(subworld: &mut SubWorld, #[resource] map: &Map) {
	let mut movers = <(&mut Point, &MovingRandomly)>::query();
	movers.iter_mut(subworld)
		.for_each(|(pos, _)| {
			let mut rng = RandomNumberGenerator::new();
			let destination = match rng.range(0, 4) {
				0 => Point::new(-1, 0),
				1 => Point::new(1, 0),
				2 => Point::new(0, -1),
				_ => Point::new(0, 1),
			} + *pos;

			if map.can_enter_tile(destination) {
				*pos = destination;
			}
			// This would be better if we got valid locations and selected from that vec
		})
}