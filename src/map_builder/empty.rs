use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect { }

impl MapArchitect for EmptyArchitect {
	fn init(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
		let mut mb = MapBuilder {
			map: Map::new(),
			rooms: Vec::new(),
			monster_spawns: Vec::new(),
			player_start: Point::zero(),
			amulet_start: Point::zero(),
		};
		mb.fill(TileType::Floor);
		mb.player_start = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
		mb.amulet_start = mb.find_most_distant();
		for _ in 0..50 {
			// If this wasn't just a trait test should prevent duplicates including amulet spawn and player spawn
			let point = Point::new(
				rng.range(1, SCREEN_WIDTH),
				rng.range(1, SCREEN_HEIGHT)
			);
			mb.monster_spawns.push(point);
		}
		mb
	}
}