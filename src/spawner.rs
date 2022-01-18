use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
	world.push(
		(
			Player,
			pos,
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph: to_cp437('@'),
			},
			Health { current: 3, max: 3 },
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

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
	let (hp, name, glyph) = match rng.roll_dice(1, 10) {
		1..=8 => goblin(),
		_ => orc()
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
		)
	);
}

fn goblin() -> (i32, String, FontCharType) {
	(1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
	(2, "Orc".to_string(), to_cp437('o'))
}