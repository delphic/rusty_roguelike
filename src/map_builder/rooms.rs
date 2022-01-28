use crate::prelude::*;
use super::MapArchitect;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
	fn init(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
		let mut mb = MapBuilder {
			map: Map::new(),
			rooms: Vec::new(),
			monster_spawns: Vec::new(),
			player_start: Point::zero(),
			amulet_start: Point::zero(),
			theme: super::themes::DungeonTheme::new(),
		};
		mb.fill(TileType::Wall);
		mb.build_random_rooms(rng);
		mb.build_cooridors(rng);
		mb.player_start = mb.rooms[0].center();
		mb.amulet_start = mb.find_most_distant();
		mb.rooms.iter().skip(1).map(|r| r.center())
			.for_each(|pos| mb.monster_spawns.push(pos));
		mb
	}
}