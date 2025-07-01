use std::error::Error;

use macroquad::prelude::*;

use crate::{editor::Editor, map::Map};

mod editor;
mod map;
mod tilemap;
mod walls;

type EvMapResult = Result<(), Box<dyn Error>>;

#[macroquad::main("EvMap")]
async fn main() -> EvMapResult {
	let mut editor = Editor::new();

	loop {
		editor.draw();
		editor.control()?;

		next_frame().await
	}
}

fn save_map(map: &Map) -> EvMapResult {
	let cfg = ron::ser::PrettyConfig::new().indentor("\t");
	let ron = ron::ser::to_string_pretty(&map, cfg)?;

	std::fs::write("map.ron", ron)?;

	Ok(())
}

pub fn mouse_pos_vec() -> Vec2 {
	let (x, y) = mouse_position();
	vec2(x, y)
}
