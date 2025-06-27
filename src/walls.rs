use macroquad::prelude::*;

use crate::{map::Map, mouse_pos_vec};

const COLORS: [Color; 10] = [
	RED, ORANGE, GOLD, YELLOW, LIME, GREEN, BLUE, PURPLE, MAROON, PINK,
];

pub fn draw_map_walls(map: &Map) {
	let mut colors = COLORS.iter().cycle();

	for wall in &map.walls {
		if wall.is_empty() {
			continue;
		}

		let mut previous = wall.first().unwrap();
		let color = *colors.next().unwrap_or(&RED);

		for point in wall {
			draw_circle(point.x, point.y, 5., color);

			if previous != point {
				draw_line(previous.x, previous.y, point.x, point.y, 2., color);
			}
			previous = point;
		}

		let first = wall.first().unwrap();
		if previous != first {
			draw_line(previous.x, previous.y, first.x, first.y, 2., color);
		}
	}
}

pub fn nearest_point(map: &Map) -> Option<usize> {
	let pos = mouse_pos_vec();
	let wall = map.walls.first()?;

	let mut dist = f32::MAX;
	let mut index = None;

	for (i, point) in wall.iter().enumerate() {
		let new_dist = point.distance(pos);

		if new_dist < dist {
			dist = new_dist;
			index = Some(i);
		}
	}

	index
}

pub fn index_to_insert_at(map: &Map) -> Option<usize> {
	let pos = mouse_pos_vec();
	let wall = map.walls.first()?;
	let index = nearest_point(map)?;

	let mut index_s = index.wrapping_sub(1);
	let mut index_l = index.wrapping_add(1);

	if wall.get(index_s).is_none() {
		index_s = wall.len();
	}
	if wall.get(index_l).is_none() {
		index_l = 0;
	}

	let point_s = wall.get(index_s)?;
	let point_l = wall.get(index_l)?;

	if point_s.distance(pos) < point_l.distance(pos) {
		Some(index_s)
	} else {
		Some(index_l)
	}
}
