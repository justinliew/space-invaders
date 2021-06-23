use crate::point::Point;
use crate::size::WorldSize;

pub struct RenderData {
	pub screen_top_left_offset: Point,
	pub game_to_screen: f64,
	pub width: usize,
	pub height: usize,
}

impl RenderData {
	pub fn new() -> Self {
		RenderData{
			screen_top_left_offset: Point::new(0.0,0.0),
			game_to_screen: 1.,
			width: 1024,
			height: 768,
		}
	}

	pub fn world_to_screen(&self, in_point: &Point) -> Point {
		Point{
			x: (in_point.x + self.screen_top_left_offset.x) * self.game_to_screen,
			y: (in_point.y + self.screen_top_left_offset.y) * self.game_to_screen,
		}
	}

	pub fn resize(&mut self, world_size: WorldSize, width: f64, height: f64) -> f64 {
		self.width = width.trunc() as usize;
		self.height = height.trunc() as usize;

		if world_size.width < width && world_size.height < height {
			self.screen_top_left_offset.x = (width - world_size.width) / 2.;
			self.screen_top_left_offset.y = (height - world_size.height) / 2.;
			self.game_to_screen = 1.;
			return self.game_to_screen;
		}

		// this stuff doesn't work very well...
		if world_size.width > width {
			self.game_to_screen = width / world_size.width;
			// this isn't quite right; it needs some sort of scaling
			self.screen_top_left_offset.y = (height - world_size.height) / 2.;
		}
		else if world_size.height > height {
			self.game_to_screen = height / world_size.height;
			// this isn't quite right; it needs some sort of scaling
			self.screen_top_left_offset.x = (width - world_size.width) / 2.;
		}
		self.game_to_screen
	}
}

