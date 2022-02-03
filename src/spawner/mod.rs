use crate::prelude::*;
use config::Config;

mod config;

pub fn spawn_level(
	world: &mut World,
	rng: &mut RandomNumberGenerator,
	level: usize,
	spawn_points: &[Point]
) {
	let template = Config::load();
	template.spawn_entities(world, rng, level, spawn_points);
}

pub fn spawn_player(world: &mut World, pos: Point) {
	world.push(
		(
			Player {
				map_level: 0,
			},
			pos,
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph: to_cp437('@'),
			},
			Health { current: 10, max: 10 },
			FieldOfView::new(8),
			Damage(1),
		)
	);
}

pub fn spawn_amulet_of_yala(world: &mut World, pos: Point) {
	world.push((
		Item,
		AmuletOfYala,
		pos,
		Render { 
			color: ColorPair::new(WHITE, BLACK),
			glyph: to_cp437('|'),
		},
		Name("Amulet of Yala".to_string())
	));
}