#[derive(Default)]
pub struct Input {
	pub any: bool,
	pub left: bool,
	pub right: bool,
	pub fire: bool,

	pub last_shoot: f64,
}