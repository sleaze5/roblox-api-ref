use std::collections::{HashMap, HashSet};

use base64::Engine;
use indexmap::IndexMap;

use crate::js::compare_names;
use crate::model::Class;

pub const CELL_SIZE: u32 = 48;
const FALLBACK_ICON: &str = "Class";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
	pub width: u32,
	pub height: u32,
	pub pixels: Vec<u8>,
}

pub struct Sprite {
	pub image: Image,
	pub columns: u32,
	pub rows: u32,
	pub cells: HashMap<String, u32>,
}

pub struct ClassIconFiles {
	pub png: Vec<u8>,
	pub css: String,
}

pub fn decode_png(bytes: &[u8]) -> Result<Image, String> {
	let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
	decoder.set_transformations(png::Transformations::normalize_to_color8() | png::Transformations::ALPHA);
	let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
	let mut pixels = vec![0; reader.output_buffer_size().ok_or("The PNG is too large.")?];
	let info = reader.next_frame(&mut pixels).map_err(|error| error.to_string())?;
	if info.color_type != png::ColorType::Rgba {
		return Err("Only RGBA PNG images are supported.".to_owned());
	}
	pixels.truncate(info.buffer_size());
	Ok(Image {
		width: info.width,
		height: info.height,
		pixels,
	})
}

pub fn encode_png(image: &Image) -> Vec<u8> {
	let mut bytes = Vec::new();
	let mut encoder = png::Encoder::new(&mut bytes, image.width, image.height);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);
	encoder.set_compression(png::Compression::High);
	encoder.set_filter(png::Filter::Adaptive);
	let mut writer = encoder.write_header().expect("an in-memory PNG header writes");
	writer
		.write_image_data(&image.pixels)
		.expect("the sprite matches its declared size");
	writer.finish().expect("an in-memory PNG finishes");
	bytes
}

fn draw(target: &mut Image, icon: &Image, cell: u32, columns: u32) -> Result<(), String> {
	if icon.width > CELL_SIZE || icon.height > CELL_SIZE {
		return Err(format!("Studio icons must fit in {CELL_SIZE}x{CELL_SIZE} pixels."));
	}
	let left = (cell % columns) * CELL_SIZE + (CELL_SIZE - icon.width) / 2;
	let top = (cell / columns) * CELL_SIZE + (CELL_SIZE - icon.height) / 2;
	let row_bytes = (icon.width * 4) as usize;
	for row in 0..icon.height {
		let from = row as usize * row_bytes;
		let to = (((top + row) * target.width + left) * 4) as usize;
		target.pixels[to..to + row_bytes].copy_from_slice(&icon.pixels[from..from + row_bytes]);
	}
	Ok(())
}

pub fn build_sprite(icons: &HashMap<String, Image>) -> Result<Sprite, String> {
	let mut names: Vec<&String> = icons.keys().collect();
	names.sort_by(|a, b| compare_names(a, b));
	let count = u32::try_from(names.len()).map_err(|error| error.to_string())?;
	let columns = count.isqrt() + u32::from(count.isqrt() * count.isqrt() < count);
	let columns = columns.max(2);
	let rows = count.div_ceil(columns).max(2);
	let width = columns * CELL_SIZE;
	let height = rows * CELL_SIZE;
	let mut image = Image {
		width,
		height,
		pixels: vec![0; (width * height * 4) as usize],
	};
	let mut cells = HashMap::new();
	for (cell, name) in (0..).zip(names) {
		draw(&mut image, &icons[name], cell, columns)?;
		cells.insert(name.clone(), cell);
	}
	Ok(Sprite {
		image,
		columns,
		rows,
		cells,
	})
}

fn own_or_inherited(classes: &IndexMap<String, Class>, item: &Class, icons: &HashSet<&str>) -> Option<String> {
	let mut current = Some(item);
	while let Some(class) = current {
		if icons.contains(class.name.as_str()) {
			return Some(class.name.clone());
		}
		current = class.superclass.as_ref().and_then(|name| classes.get(name));
	}
	icons.contains(FALLBACK_ICON).then(|| FALLBACK_ICON.to_owned())
}

pub fn resolve_class_icons(
	classes: &IndexMap<String, Class>,
	icons: &HashSet<&str>,
) -> IndexMap<String, String> {
	classes
		.values()
		.filter_map(|item| {
			own_or_inherited(classes, item, icons).map(|icon| (item.name.clone(), icon))
		})
		.collect()
}

pub fn apply_class_icons(
	classes: &mut IndexMap<String, Class>,
	studio_icons: &HashMap<String, String>,
) -> Result<ClassIconFiles, String> {
	let available: HashSet<&str> = studio_icons.keys().map(String::as_str).collect();
	let resolved = resolve_class_icons(classes, &available);
	let mut images = HashMap::new();
	for name in resolved.values() {
		if images.contains_key(name) {
			continue;
		}
		let bytes = base64::engine::general_purpose::STANDARD
			.decode(studio_icons.get(name).map_or("", String::as_str))
			.map_err(|error| error.to_string())?;
		images.insert(name.clone(), decode_png(&bytes)?);
	}
	let sprite = build_sprite(&images)?;
	for (class_name, icon) in &resolved {
		if let (Some(item), Some(cell)) = (classes.get_mut(class_name), sprite.cells.get(icon)) {
			item.icon = Some(*cell);
		}
	}
	Ok(ClassIconFiles {
		png: encode_png(&sprite.image),
		css: format!(
			":root{{--class-icon-columns:{};--class-icon-rows:{}}}\n",
			sprite.columns, sprite.rows
		),
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	fn solid(width: u32, height: u32, value: u8) -> Image {
		Image {
			width,
			height,
			pixels: vec![value; (width * height * 4) as usize],
		}
	}

	fn pixel(image: &Image, x: u32, y: u32) -> u8 {
		image.pixels[((y * image.width + x) * 4) as usize]
	}

	#[test]
	fn places_icons_in_name_order_on_a_square_grid_and_centers_short_icons() {
		let sprite = build_sprite(&HashMap::from([
			("Part".to_owned(), solid(CELL_SIZE, CELL_SIZE, 1)),
			("Model".to_owned(), solid(CELL_SIZE, CELL_SIZE, 2)),
			("Folder".to_owned(), solid(CELL_SIZE, CELL_SIZE / 2, 3)),
		]))
		.unwrap();
		assert_eq!(sprite.cells["Folder"], 0);
		assert_eq!(sprite.cells["Model"], 1);
		assert_eq!(sprite.cells["Part"], 2);
		assert_eq!((sprite.columns, sprite.rows), (2, 2));
		assert_eq!(pixel(&sprite.image, CELL_SIZE, 0), 2);
		assert_eq!(pixel(&sprite.image, 0, CELL_SIZE), 1);
		assert_eq!(pixel(&sprite.image, 0, 0), 0);
		assert_eq!(pixel(&sprite.image, 0, CELL_SIZE / 4), 3);
		assert_eq!(pixel(&sprite.image, 0, CELL_SIZE * 3 / 4), 0);
	}

	#[test]
	fn round_trips_png_images() {
		let mut image = solid(5, 6, 0);
		for (index, byte) in image.pixels.iter_mut().enumerate() {
			*byte = u8::try_from((index * 37 + (index >> 3) * 11) & 0xff).unwrap();
		}
		assert_eq!(decode_png(&encode_png(&image)).unwrap(), image);
	}

	#[test]
	fn computes_square_grids() {
		for (count, columns) in [(1, 2), (4, 2), (5, 3), (9, 3), (10, 4), (289, 17)] {
			let icons: HashMap<String, Image> = (0..count)
				.map(|index| (format!("C{index}"), solid(1, 1, 0)))
				.collect();
			assert_eq!(build_sprite(&icons).unwrap().columns, columns);
		}
	}
}
