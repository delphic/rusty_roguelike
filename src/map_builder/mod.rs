mod automata;
mod drunkard;
mod prefab;
mod rooms;
mod themes;

use crate::prelude::*;
use rooms::RoomsArchitect;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use std::cmp;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
	fn init(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme : Sync + Send {
	fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

pub struct MapBuilder {
	pub map : Map,
	pub rooms : Vec<Rect>,
	pub monster_spawns : Vec<Point>,
	pub player_start : Point,
	pub amulet_start : Point,
	pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
	pub fn new(rng: &mut RandomNumberGenerator) -> Self {
		let mut architect : Box<dyn MapArchitect> = match rng.range(0, 3) {
			0 => Box::new(DrunkardsWalkArchitect {}),
			1 => Box::new(CellularAutomataArchitect {}),
			_ => Box::new(RoomsArchitect {}),
		};
		let mut mb = architect.init(rng);
		prefab::apply_prefab(&mut mb, rng); // Add a FORTRESS

		mb.theme = match rng.range(0, 2) {
			0 => themes::DungeonTheme::new(),
			_ => themes::ForestTheme::new(),
		};
		mb
	}

	fn fill(&mut self, tile: TileType) {
		self.map.tiles.iter_mut().for_each(|t| *t = tile);
	}

	fn find_most_distant(&self) -> Point {
		let dijkstra_map = DijkstraMap::new(
			SCREEN_WIDTH,
			SCREEN_HEIGHT,
			&vec![self.map.point2d_to_index(self.player_start)],
			&self.map,
			1024.0
		);

		const UNREACHABLE : &f32 = &f32::MAX;
		self.map.index_to_point2d(
			dijkstra_map.map
				.iter()
				.enumerate()
				.filter(|(_,dist)| *dist < UNREACHABLE)
				.max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
				.unwrap().0
		)
	}

	fn find_monster_spawns(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
		const NUM_MONSTERS : usize = 50;

		let mut spawnable_tiles : Vec<Point> = self.map.tiles.iter().enumerate()
			.filter(|(idx, t)| **t == TileType::Floor && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx)) > 10.0)
			.map(|(idx, _)| self.map.index_to_point2d(idx))
			.collect();

		let mut spawns = Vec::new();
		for _ in 0 .. cmp::min(NUM_MONSTERS, spawnable_tiles.len()) {
			let target_index = rng.random_slice_index(&spawnable_tiles).unwrap(); 
			spawns.push(spawnable_tiles[target_index].clone());
			spawnable_tiles.remove(target_index);
		}
		spawns
	}

	// Arguably these should live in rooms.rs - waiting to see if the author reuses them
	fn build_random_rooms(&mut self, rng : &mut RandomNumberGenerator) {
		while self.rooms.len() < NUM_ROOMS {
			let room = Rect::with_size(
				rng.range(1, SCREEN_WIDTH - 10),
				rng.range(1, SCREEN_HEIGHT - 10),
				rng.range(2, 10),
				rng.range(2, 10),
			);
			let mut overlap = false;
			for r in self.rooms.iter() {
				if r.intersect(&room) {
					overlap = true;
				}
			}
			// ^^ Guilherme style brute force eh... well presumably for a large enough screen size it'll always have a solution
			// but it'll take a variable amount of time to generate - so yes I hate this approach - would prefer letting them overlap frankly
			if !overlap {
				room.for_each(|p| {
					if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
						let idx = get_map_idx(p.x, p.y);
						self.map.tiles[idx] = TileType::Floor;
					}
				});
				self.rooms.push(room);
			}
		}
	}

	fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
		use std::cmp::{min, max};
		for y in min(y1, y2) ..= max(y1, y2) {
			if let Some(idx) = self.map.try_get_map_idx(Point::new(x, y)) {
				self.map.tiles[idx as usize] = TileType::Floor;
			}
		}
	}

	fn apply_horizonal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
		use std::cmp::{min, max};
		for x in min(x1, x2) ..= max(x1, x2) {
			if let Some(idx) = self.map.try_get_map_idx(Point::new(x, y)) {
				self.map.tiles[idx as usize] = TileType::Floor;
			}
		}
	}

	fn build_cooridors(&mut self, rng: &mut RandomNumberGenerator) {
		let mut rooms = self.rooms.clone();
		rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

		for (i, room) in rooms.iter().enumerate().skip(1) {
			let prev = rooms[i-1].center();
			let new = room.center();

			if rng.range(0, 2) == 1 {
				self.apply_horizonal_tunnel(prev.x, new.x, prev.y);
				self.apply_vertical_tunnel(prev.y, new.y, new.x);
			} else {
				self.apply_vertical_tunnel(prev.y, new.y, prev.x);
				self.apply_horizonal_tunnel(prev.x, new.x, new.y);
			}
		}
	}
}