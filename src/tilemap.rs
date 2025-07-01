use std::collections::HashMap;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub type Sprites = HashMap<String, Texture2D>;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TileMap(HashMap<i8, Vec<(Vec2, String)>>);

impl TileMap {
	pub fn render(&self, sprites: &Sprites) {
		(i8::MIN..i8::MAX)
			.filter_map(|ref i| self.0.get(i))
			.for_each(|vec| {
				for (pos, key) in vec {
					if let Some(texture) = sprites.get(key) {
						draw(texture, pos);
					}
				}
			})
	}
}

fn draw(texture: &Texture2D, pos: &Vec2) {
	fn pixel_offset(base: f32) -> f32 {
		const SCREEN_SCALE: f32 = 3.;
		(base / SCREEN_SCALE).round() * SCREEN_SCALE
	}

	draw_texture(texture, pixel_offset(pos.x), pixel_offset(pos.y), WHITE);
}
