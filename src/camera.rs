use crate::prelude::*;

pub struct Camera {
	pub left_x : i32,
	pub right_x : i32,
	pub top_y : i32,
	pub bottom_y: i32,
}

impl Camera {
	pub fn new(initial_position: Point) -> Self {
		let mut camera = Self { left_x: 0, right_x: 0, top_y: 0, bottom_y: 0 };
		camera.set_position(initial_position);
		camera
	}

	pub fn set_position(&mut self, position: Point) {
		self.left_x = position.x - DISPLAY_WIDTH/2;
		self.right_x = position.x - DISPLAY_WIDTH/2 + DISPLAY_WIDTH;
		self.top_y = position.y - DISPLAY_HEIGHT/2;
		self.bottom_y = position.y - DISPLAY_HEIGHT/2 + DISPLAY_HEIGHT;
	}
}