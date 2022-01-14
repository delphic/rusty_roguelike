use crate::prelude::*;

const LAYER : usize = 3;
const SORT_ORDER : usize = 10000;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(sub_world: &SubWorld) {
	let mut health_query = <&Health>::query().filter(component::<Player>()); 
	// ^^ Why not just <(Player, &Health)>::query ? maybe we're going to look at 
	// enemies later?

	let player_health = health_query.iter(sub_world)
		.nth(0)
		.unwrap();

	let mut draw_batch = DrawBatch::new();
	draw_batch.target(LAYER);
	draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
	draw_batch.bar_horizontal(
		Point::zero(),
		SCREEN_WIDTH * 2,
		player_health.current,
		player_health.max,
		ColorPair::new(RED, BLACK)
	);
	draw_batch.print_color_centered(
		0,
		format!(" Health: {} / {}", player_health.current, player_health.max),
		ColorPair::new(WHITE, RED)
	);
	draw_batch.submit(SORT_ORDER).expect("Batch error");
}