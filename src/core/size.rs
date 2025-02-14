/// A `WorldSize` represents a region in screenspace
#[derive(Clone, Copy, Default)]
pub struct WorldSize {
    pub width: f64,
    pub height: f64
}

impl WorldSize {
	pub fn new(w: f64, h: f64) -> Self {
		WorldSize{
			width: w,
			height: h,
		}
	}
}

/// A `Size` represents a region in screenspace
#[derive(Clone, Copy, Default)]
pub struct ScreenSize {
    pub _width: usize,
    pub _height: usize
}
