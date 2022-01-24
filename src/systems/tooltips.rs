use crate::prelude::*;

const LAYER : usize = 3;
const SORT_ORDER : usize = 10100;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(
	sub_world: &SubWorld,
	#[resource] mouse_pos: &Point,
	#[resource] camera: &Camera
) {
	let mut positions = <(Entity, &Point, &Name)>::query();
	let mut fov = <&FieldOfView>::query().filter(component::<Player>());
	let player_fov = fov.iter(sub_world).nth(0).unwrap();

	let offset = Point::new(camera.left_x, camera.top_y);
	let map_pos = *mouse_pos + offset;
	let mut draw_batch = DrawBatch::new();
	draw_batch.target(LAYER);
	positions
		.iter(sub_world)
		.filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(pos))
		.for_each(|(entity, _, name)| {
			let screen_pos = *mouse_pos * 4;
			let display = if let Ok(health) = sub_world.entry_ref(*entity)
				.unwrap()
				.get_component::<Health>()
				{
				format!("{} : {} hp", &name.0, health.current)
			} else {
				name.0.clone()
			};
			draw_batch.print(screen_pos, &display);
		});
	draw_batch.submit(SORT_ORDER).expect("Batch error");
}