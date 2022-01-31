use crate::prelude::*;

const LAYER : usize = 3;
const SORT_ORDER : usize = 10000;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Name)]
#[read_component(Carried)]
pub fn hud(sub_world: &SubWorld) {
	let mut health_query = <&Health>::query().filter(component::<Player>()); 
	// ^^ Why not just <(Player, &Health)>::query ? maybe we're going to look at 
	// enemies later?

	let player_health = health_query.iter(sub_world)
		.nth(0)
		.unwrap();

	let mut draw_batch = DrawBatch::new();
	draw_batch.target(LAYER);

	// Display Heatlh
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

	// Prompt
	draw_batch.print_centered(1, "Explore the Dungeon. Use arrow keys to move.");

	// Display Current level
	let (_player, map_level) = <(Entity, &Player)>::query().iter(sub_world).find_map(|(entity, player)| Some((*entity, player.map_level))).unwrap();
	draw_batch.print_color_right(
		Point::new(SCREEN_WIDTH*2, 1),
		format!("Dungeon Level: {}", map_level+1),
		ColorPair::new(YELLOW, BLACK));

	// Display inventory
	let player = <(Entity, &Player)>::query().iter(sub_world).find_map(|(entity, _player)| Some(*entity)).unwrap();
	let mut inventory_query = <(&Item, &Name, &Carried)>::query();
	let mut y = 3;
	inventory_query.iter(sub_world).filter(|(_, _, carried)| carried.0 == player)
		.for_each(|(_, name, _)| {
			draw_batch.print(
				Point::new(3, y),
				format!("{}: {}", y-2, &name.0)
			);
			y += 1;
		});
	if y > 3 {
		draw_batch.print_color(Point::new(3, 2), "Items carried", ColorPair::new(YELLOW, BLACK));
	}

	draw_batch.submit(SORT_ORDER).expect("Batch error");
}