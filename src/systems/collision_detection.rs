use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collision_detection(sub_world: &mut SubWorld, commands: &mut CommandBuffer) {

	let mut player_pos = Point::zero();
	<&Point>::query().filter(component::<Player>())
		.iter(sub_world)
		.for_each(|pos| player_pos = *pos);

	let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

	enemies.iter(sub_world)
		.filter(|(_, pos)| **pos == player_pos) // could also do |(_, &pos)| *pos == player_pos - it's probably more readable
		.for_each(|(entity, _)| {
			commands.remove(*entity);
		});
}