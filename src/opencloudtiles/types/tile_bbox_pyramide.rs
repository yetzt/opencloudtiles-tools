use std::fmt;

use super::{TileBBox, TileCoord2, TileCoord3};

const MAX_ZOOM_LEVEL: usize = 32;

pub struct TileBBoxPyramide {
	level_bbox: [TileBBox; MAX_ZOOM_LEVEL],
}

impl TileBBoxPyramide {
	pub fn new_full() -> TileBBoxPyramide {
		TileBBoxPyramide {
			level_bbox: std::array::from_fn(|z| TileBBox::new_full(z as u64)),
		}
	}
	pub fn new_empty() -> TileBBoxPyramide {
		TileBBoxPyramide {
			level_bbox: std::array::from_fn(|_z| TileBBox::new_empty()),
		}
	}
	pub fn set_zoom_min(&mut self, zoom_level_min: u64) {
		for (index, bbox) in self.level_bbox.iter_mut().enumerate() {
			let level = index as u64;
			if level < zoom_level_min {
				bbox.set_empty();
			}
		}
	}
	pub fn set_zoom_max(&mut self, zoom_level_max: u64) {
		for (index, bbox) in self.level_bbox.iter_mut().enumerate() {
			let level = index as u64;
			if level > zoom_level_max {
				bbox.set_empty();
			}
		}
	}
	pub fn limit_by_geo_bbox(&mut self, geo_bbox: &[f32; 4]) {
		for (level, bbox) in self.level_bbox.iter_mut().enumerate() {
			bbox.intersect(&TileBBox::from_geo(level as u64, geo_bbox));
		}
	}
	pub fn intersect(&mut self, other_bbox_pyramide: &TileBBoxPyramide) {
		for (level, bbox) in self.level_bbox.iter_mut().enumerate() {
			let other_bbox = other_bbox_pyramide.get_level_bbox(level as u64);
			bbox.intersect(other_bbox);
		}
	}
	pub fn get_level_bbox(&self, level: u64) -> &TileBBox {
		return &self.level_bbox[level as usize];
	}
	pub fn set_level_bbox(&mut self, level: u64, bbox: TileBBox) {
		self.level_bbox[level as usize] = bbox;
	}
	pub fn include_coord(&mut self, coord: &TileCoord3) {
		self.level_bbox[coord.z as usize].include_tile(coord.x, coord.y);
	}
	pub fn include_bbox(&mut self, level: u64, bbox: &TileBBox) {
		self.level_bbox[level as usize].include_bbox(bbox);
	}
	pub fn iter_levels(&self) -> impl Iterator<Item = (u64, &TileBBox)> {
		return self
			.level_bbox
			.iter()
			.enumerate()
			.filter_map(|(level, bbox)| {
				if bbox.is_empty() {
					return None;
				} else {
					return Some((level as u64, bbox));
				}
			});
	}
	pub fn iter_tile_indexes(&self) -> impl Iterator<Item = TileCoord3> + '_ {
		return self
			.level_bbox
			.as_slice()
			.iter()
			.enumerate()
			.map(|(z, bbox)| {
				bbox
					.iter_coords()
					.map(move |TileCoord2 { x, y }| TileCoord3 { x, y, z: z as u64 })
			})
			.flatten();
	}
	pub fn get_max_zoom(&self) -> u64 {
		let mut max: usize = 0;
		for (level, bbox) in self.level_bbox.iter().enumerate() {
			if !bbox.is_empty() {
				max = level;
			}
		}
		return max as u64;
	}
	pub fn count_tiles(&self) -> u64 {
		return self.level_bbox.iter().map(|bbox| bbox.count_tiles()).sum();
	}
}

impl fmt::Debug for TileBBoxPyramide {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list()
			.entries(
				self
					.iter_levels()
					.map(|(level, bbox)| format!("{}: {:?}", level, bbox)),
			)
			.finish()
	}
}
