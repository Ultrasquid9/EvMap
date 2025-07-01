use std::{collections::HashMap, path::Path};

use macroquad::prelude::*;
use walkdir::WalkDir;

use crate::{
	DynResult,
	map::Map,
	mouse_pos_vec, save_map,
	tilemap::Sprites,
	walls::{draw_map_walls, index_to_insert_at, nearest_point},
};

#[derive(Default)]
pub struct Editor {
	pub map: Map,
	pub drag_index: Option<usize>,
	pub sprites: Sprites,
}

impl Editor {
	pub async fn new() -> Self {
		let mut map = Map::default();
		map.walls.push(vec![]);

		Self {
			map,
			sprites: load_sprites().await.unwrap_or_default(),
			..Default::default()
		}
	}

	pub async fn with_file(dir: impl AsRef<Path>) -> DynResult<Self> {
		let string = std::fs::read_to_string(dir)?;
		let map = ron::from_str::<Map>(&string)?;

		Ok(Self {
			map,
			sprites: load_sprites().await.unwrap_or_default(),
			..Default::default()
		})
	}

	pub async fn control(&mut self) -> DynResult<()> {
		if is_mouse_button_pressed(MouseButton::Left) {
			let index = index_to_insert_at(&self.map).unwrap_or_default();
			self.map
				.walls
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

		if is_key_pressed(KeyCode::Escape) {
			self.map = Map::default();
			self.map.walls.push(vec![]);
		}
		if is_key_pressed(KeyCode::R) {
			self.sprites = load_sprites().await?;
		}
		if is_key_pressed(KeyCode::S) {
			save_map(&self.map)?;
		}

		Ok(())
	}

	pub fn draw(&self) {
		self.map.tilemap.render(&self.sprites);
		draw_map_walls(&self.map);
	}
}

async fn load_sprites() -> DynResult<Sprites> {
	let mut sprites = HashMap::new();

	for entry in std::fs::read_dir("cores")? {
		let entry = entry?;
		let path = entry.path().join("sprites");
		let core_name = entry.file_name().to_string_lossy().to_string();

		for entry in WalkDir::new(path) {
			let entry = entry?;
			let path = entry.path().as_os_str().to_string_lossy();
			let Ok(texture) = load_texture(&path).await else {
				continue;
			};

			sprites.insert(gen_name(&core_name, &path), texture);
		}
	} 
	
	Ok(sprites)
}

fn gen_name(core: &str, path: &str) -> String {
	let (_, str) = path.split_once(&core).unwrap();
	let (_, str) = str.split_once("/sprites/").unwrap();
	let mut vec = str.split('.').collect::<Vec<&str>>();
	
	vec.pop();
	core.to_string() + ":" + &vec.join(".")
}
