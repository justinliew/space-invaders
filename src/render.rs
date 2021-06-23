use crate::point::Point;

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
}
