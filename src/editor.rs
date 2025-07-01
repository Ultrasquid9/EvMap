use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{map::Map, mouse_pos_vec, save_map, walls::{draw_map_walls, index_to_insert_at, nearest_point}, EvMapResult};

pub struct Editor {
	pub map: Map,
	pub drag_index: Option<usize>
}

impl Editor {
	pub fn new() -> Self {
		let mut map = Map::default();
		map.walls.push(vec![]);

		Self {
			map,
			drag_index: None,
		}
	}

	pub fn control(&mut self) -> EvMapResult {
		if is_mouse_button_pressed(MouseButton::Left) {
			let index = index_to_insert_at(&self.map).unwrap_or_default();
			self.map.walls
				.first_mut()
				.unwrap()
				.insert(index, mouse_pos_vec());
		}

		if is_mouse_button_pressed(MouseButton::Right) {
			self.drag_index = nearest_point(&self.map);
		}

		if is_mouse_button_down(MouseButton::Right)
			&& let Some(index) = self.drag_index
			&& let Some(wall) = self.map.walls.first_mut()
			&& let Some(point) = wall.get_mut(index)
		{
			*point = mouse_pos_vec();
		}

		if is_mouse_button_pressed(MouseButton::Middle)
			&& let Some(point) = nearest_point(&self.map)
			&& let Some(wall) = self.map.walls.first_mut()
		{
			wall.remove(point);
		}

		if is_key_pressed(KeyCode::S) {
			save_map(&self.map)?;
		}

		Ok(())
	}

	pub fn draw(&self) {
		draw_map_walls(&self.map);
		self.map.tilemap.render(HashMap::new());
	}
}
