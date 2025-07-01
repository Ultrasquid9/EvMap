use std::error::Error;

use macroquad::prelude::*;

use crate::{editor::Editor, map::Map};

mod editor;
mod map;
mod tilemap;
mod walls;

pub type DynResult<T> = Result<T, Box<dyn Error>>;

#[macroquad::main("EvMap")]
async fn main() -> DynResult<()> {
	let mut editor = Editor::with_file("map.ron")
		.await
		.unwrap_or(Editor::new().await);

	loop {
		editor.draw();
		editor.control().await?;

		next_frame().await
	}
}

fn save_map(map: &Map) -> DynResult<()> {
	let cfg = ron::ser::PrettyConfig::new().indentor("\t");
	let ron = ron::ser::to_string_pretty(&map, cfg)?;

	std::fs::write("map.ron", ron)?;

	Ok(())
}

pub fn mouse_pos_vec() -> Vec2 {
	let (x, y) = mouse_position();
	vec2(x, y)
}
