use std::error::Error;

use macroquad::prelude::*;

use crate::{
	map::Map,
	walls::{draw_map_walls, index_to_insert_at, nearest_point},
};

mod map;
mod walls;

type EvMapResult = Result<(), Box<dyn Error>>;

#[macroquad::main("EvMap")]
async fn main() -> EvMapResult {
	let mut map = Map::default();
	map.walls.push(vec![]);

	let mut drag_index: Option<usize> = None;

	loop {
		draw_map_walls(&map);

		if is_mouse_button_pressed(MouseButton::Left) {
			let index = index_to_insert_at(&map).unwrap_or_default();
			map.walls
				.first_mut()
				.unwrap()
				.insert(index, mouse_pos_vec());
		}

		if is_mouse_button_pressed(MouseButton::Right) {
			drag_index = nearest_point(&map);
		}

		if is_mouse_button_down(MouseButton::Right)
			&& let Some(index) = drag_index
			&& let Some(wall) = map.walls.first_mut()
			&& let Some(point) = wall.get_mut(index)
		{
			*point = mouse_pos_vec();
		}

		if is_mouse_button_pressed(MouseButton::Middle)
			&& let Some(point) = nearest_point(&map)
			&& let Some(wall) = map.walls.first_mut()
		{
			wall.remove(point);
		}

		if is_key_pressed(KeyCode::S) {
			save_map(&map)?;
		}

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
