// I'm unsure of the wisdom of having a single file called components but okay lets see how we go
pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
	pub color: ColorPair,
	pub glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;