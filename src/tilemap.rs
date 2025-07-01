use std::collections::HashMap;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TileMap(HashMap<i8, Vec<(Vec2, String)>>);

impl TileMap {
	pub fn render(&self, textures: HashMap<String, Texture2D>) {
		(i8::MIN..i8::MAX)
			.filter_map(|ref i| self.0.get(i))
			.for_each(|vec| {
				for (pos, key) in vec {
					textures.get(key).map(|t| draw(t, pos));
				}
			})
	}
}

fn draw(texture: &Texture2D, pos: &Vec2) {
	draw_texture(texture, pos.x, pos.y, WHITE);
}
