/// A `WorldSize` represents a region in screenspace
#[derive(Clone, Copy, Default)]
pub struct WorldSize {
    pub width: f64,
    pub height: f64
}

/// A `Size` represents a region in screenspace
#[derive(Clone, Copy, Default)]
pub struct ScreenSize {
    pub width: usize,
    pub height: usize
}
