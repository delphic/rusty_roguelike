use crate::prelude::*;

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
			Health { current: 3, max: 3 },
			FieldOfView::new(8),
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

// TODO: Replace with loot table with maximums (e.g. 1 map, 3 potions)
pub fn spawn_entity(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
	let roll = rng.roll_dice(1,12);
	match roll {
		1 => spawn_healing_potion(world, pos),
		2 => spawn_magic_map(world, pos),
		_ => spawn_monster(world, rng, pos),
	}
}

pub fn spawn_healing_potion(world: &mut World, pos: Point) {
	world.push((
		Item,
		pos,
		Render {
			color: ColorPair::new(WHITE, BLACK),
			glyph: to_cp437('!')
		},
		Name("Healing Potion".to_string()),
		ProvidesHealing{ amount: 2 },
	));
}

pub fn spawn_magic_map(world: &mut World, pos: Point) {
	world.push(( 
		Item,
		pos,
		Render {
			color: ColorPair::new(WHITE, BLACK),
			glyph: to_cp437('{')
		},
		Name("Dungeon Map".to_string()),
		ProvidesDungeonMap {},
	));
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
	let (hp, name, glyph) = match rng.roll_dice(1, 10) {
		1..=8 => goblin(),
		_ => claws()
	};

	world.push(
		(
			Enemy,
			pos,
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph
			},
			ChasingPlayer{},
			Health { current: hp, max: hp },
			Name(name),
			FieldOfView::new(6),
		)
	);
}

fn goblin() -> (i32, String, FontCharType) {
	(1, "Goblin".to_string(), to_cp437('g'))
}

fn claws() -> (i32, String, FontCharType) {
	(2, "Claw Monster".to_string(), to_cp437('o'))
}