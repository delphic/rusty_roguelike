use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
	fn init(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
		let mut mb = MapBuilder {
			map: Map::new(),
			rooms: Vec::new(),
			monster_spawns: Vec::new(),
			player_start: Point::zero(),
			amulet_start: Point::zero(),
			theme: super::themes::DungeonTheme::new(),
		};

		self.generate_random_noise_map(rng, &mut mb.map);
		for _ in 0..10 {
			self.iterate(&mut mb.map);
		}
		self.enclose(&mut mb.map);

		mb.player_start = self.find_start(&mb.map);
		mb.monster_spawns = mb.find_monster_spawns(&mb.player_start, rng);
		mb.amulet_start = mb.find_most_distant();

		mb
	}
}

impl CellularAutomataArchitect {
	fn generate_random_noise_map(
		&mut self, 
		rng: &mut RandomNumberGenerator,
		map: &mut Map
	) {
		map.tiles.iter_mut().for_each(|t| {
			let roll = rng.range(0, 100);
			if roll > 55 {
				*t = TileType::Floor;
			} else {
				*t = TileType::Wall;
			}
		});
	}

	fn iterate(&mut self, map: &mut Map) {
		let mut new_tiles = map.tiles.clone();
		for y in 1 .. SCREEN_HEIGHT - 1 {
			for x in 1 .. SCREEN_WIDTH - 1 {
				let neighbours = self.count_neighbours(x, y, map);
				let idx = get_map_idx(x, y);
				if neighbours > 4 || neighbours == 0 {
					new_tiles[idx] = TileType::Wall;
				} else {
					new_tiles[idx] = TileType::Floor;
				}
			}
		}
		map.tiles = new_tiles;
	}

	fn enclose(&mut self, map: &mut Map) {
		for x in 0 .. SCREEN_WIDTH {
			map.tiles[get_map_idx(x, 0)] = TileType::Wall;
			map.tiles[get_map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
		}
		for y in 1 .. SCREEN_HEIGHT - 1 {
			map.tiles[get_map_idx(0, y)] = TileType::Wall;
			map.tiles[get_map_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
		}
	}

	fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
		let mut neighbours = 0;
		for iy in -1..=1 {
			for ix in -1..=1 {
				if !(ix == 0 && iy == 0) && map.tiles[get_map_idx(x + ix, y + iy)] == TileType::Wall {
					neighbours += 1;
				}
			}
		}
		neighbours
	}

	fn find_start(&self, map: &Map) -> Point {
		// Arguably should use flow maps to determine connected spaces - we do that for the amulet at least,
		// so worst case scenario you spawn in a tiny room with the amulet (although if 1x1 rooms are possible it could be a problem!)
		let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
		let closest_point = map.tiles.iter().enumerate()
			.filter(|(_, t)| **t == TileType::Floor)
			.map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx))))
			.min_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap()) // compare distances - we know the floats are valid so just unwrap
			.map(|(idx, _)| idx)
			.unwrap();
		map.index_to_point2d(closest_point)
	}
}