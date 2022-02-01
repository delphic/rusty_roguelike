use crate::prelude::*;
use super::MapArchitect;
pub struct DrunkardsWalkArchitect {}

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR_COUNT: usize = NUM_TILES / 3;

impl MapArchitect for DrunkardsWalkArchitect {
	fn init(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
		let mut mb = MapBuilder {
			map: Map::new(),
			rooms: Vec::new(),
			spawn_points: Vec::new(),
			player_start: Point::zero(),
			amulet_start: Point::zero(),
			theme: super::themes::DungeonTheme::new(),
		};

		mb.fill(TileType::Wall);
		let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
		match rng.range(0,5) {
			0 => self.perform_chained_drunkards(&center, rng, &mut mb),
			_ => self.perform_random_drunkards(&center, rng, &mut mb),
		}

		mb.spawn_points = mb.find_monster_spawns(&center, rng);
		mb.player_start = center;
		mb.amulet_start = mb.find_most_distant();
		mb
	}
}

impl DrunkardsWalkArchitect {
	// Adds new drunkards randomly over map - checks tiles connect to center tile and fills in if they do not
	// This presumably produces more connected caverns vibe, but likely to be inefficient for large maps 
	// Could try picking within a certain distance of existing tunnels for larger maps
	fn perform_random_drunkards(&mut self, center: &Point, rng: &mut RandomNumberGenerator, mb: &mut MapBuilder) {
		self.drunkard(center, rng, &mut mb.map);
		while mb.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR_COUNT {
			
			let random_point = Point::new(rng.range(1, SCREEN_WIDTH), rng.range(1, SCREEN_HEIGHT));
			self.drunkard(&random_point, rng, &mut mb.map);
			
			let dijkstra_map = DijkstraMap::new(
				SCREEN_WIDTH,
				SCREEN_HEIGHT,
				&vec![mb.map.point2d_to_index(*center)],
				&mb.map,
				1024.0);
			
			dijkstra_map.map.iter().enumerate()
				.filter(|(_, distance)| *distance > &2000.0)
				.for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
		}
	}

	// Adds a new drunkard randomly within the area carved about by the last - ensuring navigability inherently
	// however makes it more likely to carve out large areas with more iterations 
	// Might be interesting to prioritise distant tiles when selecting from valid tiles?
	fn perform_chained_drunkards(&mut self, center: &Point, rng: &mut RandomNumberGenerator, mb: &mut MapBuilder) {
		self.drunkard(center, rng, &mut mb.map);
		while mb.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR_COUNT {
			let dijkstra_map = DijkstraMap::new(
				SCREEN_WIDTH,
				SCREEN_HEIGHT,
				&vec![mb.map.point2d_to_index(*center)],
				&mb.map,
				1024.0);
			
			let valid_tiles : Vec<usize> = dijkstra_map.map.iter().enumerate()
				.filter(|(_, distance)| **distance < 1024.0)
				.map(|(idx, _)| idx)
				.collect();

			let slice_idx = rng.random_slice_index(&valid_tiles).unwrap();
			let map_idx = valid_tiles[slice_idx];
			self.drunkard(&mb.map.index_to_point2d(map_idx), rng, &mut mb.map);
		}
	}

	fn drunkard(
		&mut self,
		start: &Point,
		rng: &mut RandomNumberGenerator,
		map: &mut Map
	) {
		let mut drunkard_pos = start.clone();
		let mut distance_staggered = 0;

		loop {
			if !map.within_borders(drunkard_pos) {
				break;
			}

			let drunk_idx = map.point2d_to_index(drunkard_pos);
			map.tiles[drunk_idx] = TileType::Floor;
			match rng.range(0, 4) {
				0 => drunkard_pos.x -= 1,
				1 => drunkard_pos.x += 1,
				2 => drunkard_pos.y -= 1,
				_ => drunkard_pos.y += 1,
			}

			distance_staggered += 1;
			if distance_staggered > STAGGER_DISTANCE {
				break;
			}
		}
	}
}