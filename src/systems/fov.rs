use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(sub_world: &mut SubWorld, #[resource] map: &Map) {
	let mut views = <(&Point, &mut FieldOfView)>::query();
	views.iter_mut(sub_world)
		.filter(|(_, fov)| fov.is_dirty)
		.for_each(|(pos, mut fov)| {
			fov.visible_tiles = field_of_view_set(*pos, fov.radius, map); // Bracket-lib function
			fov.is_dirty = false;
		});
}