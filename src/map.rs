use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Map {
	pub walls: Vec<Vec<Vec2>>,
	pub doors: Vec<Door>,
	pub enemies: Vec<(String, Vec2)>,
	pub npcs: Vec<(String, Vec2)>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Door {
	direction: Direction,
	pos: Vec2,
	dest: String,
}
