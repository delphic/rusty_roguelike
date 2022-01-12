use crate::prelude::*;

const LAYER : usize = 2;
const SORT_ORDER : usize = 5000; // map expected to contain ~4000 elements

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(sub_world: &SubWorld, #[resource] camera: &Camera) {
	let mut draw_batch = DrawBatch::new();
	draw_batch.target(LAYER);

	let offset = Point::new(camera.left_x, camera.top_y);

	<(&Point, &Render)>::query()
		.iter(sub_world)
		.for_each(|(pos, render)| {
			draw_batch.set(*pos - offset, render.color, render.glyph);
		});
	
	draw_batch.submit(SORT_ORDER).expect("Batch error");
}