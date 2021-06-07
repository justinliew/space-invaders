// /// A `Size` represents a region in space
// #[derive(Clone, Copy, Default)]
// pub struct Size {
//     pub width: f64,
//     pub height: f64
// }

/// A `Size` represents a region in screenspace
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize
}
