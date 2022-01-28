use crate::prelude::*;

const LAYER : usize = 1;
const SORT_ORDER : usize = 0;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
	sub_world: &SubWorld,
	#[resource] map: &Map,
	#[resource] camera: &Camera,
	#[resource] theme: &Box<dyn MapTheme>
) {
	let mut fov = <&FieldOfView>::query().filter(component::<Player>());
	let player_fov = fov.iter(sub_world).nth(0).unwrap();

	let mut draw_batch = DrawBatch::new();
	draw_batch.target(LAYER);
	for y in camera.top_y ..= camera.bottom_y {
		for x in camera.left_x .. camera.right_x {
			let pt = Point::new(x, y);
			let offset = Point::new(camera.left_x, camera.top_y);
			let is_point_visible = player_fov.visible_tiles.contains(&pt);
			if map.in_bounds(pt) 
				&& (is_point_visible | map.revealed_tiles[get_map_idx(x, y)]) {
				
				let tint = if is_point_visible { 
					WHITE
				} else {
					DARK_GREY
				};

				let idx = get_map_idx(x, y);
				let glyph = theme.tile_to_render(map.tiles[idx]);
				draw_batch.set(
					pt - offset,
					ColorPair::new(tint, BLACK),
					glyph
				);
			}
		}
	}
	draw_batch.submit(SORT_ORDER).expect("Batch error");
}