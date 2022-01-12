use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
	world.push(
		(
			Player,
			pos,
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph: to_cp437('@'),
			}
		)
	);
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
	world.push(
		(
			Enemy,
			pos,
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph: match rng.range(0,4) {
					0 => to_cp437('E'),	// BUG: the shorts of the entin do not render / are transparent, which is... interesting
					1 => to_cp437('O'),
					2 => to_cp437('o'),
					_ => to_cp437('g'),
				} // TODO: Store type somehow and select glyph accordingly rather than RNGing random enemies that are all the same
			}
		)
	);
}