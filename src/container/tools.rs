use super::{abstract_classes::*, *};
use crate::Convert;
use std::path::PathBuf;

pub struct Tools;
impl Tools {
	pub fn convert(command: &Convert) -> std::io::Result<()> {
		let reader = Tools::new_reader(&command.input_file, command)?;
		let mut converter = Tools::new_converter(&command.output_file, command)?;
		converter.convert_from(reader)?;

		return Ok(());
	}
	pub fn new_reader(filename: &PathBuf, _command: &Convert) -> std::io::Result<Box<dyn Reader>> {
		let extension = filename.extension().unwrap().to_str();
		let reader = match extension {
			Some("mbtiles") => mbtiles::Reader::load(filename)?,
			Some("cloudtiles") => cloudtiles::Reader::load(filename)?,
			_ => panic!("extension '{:?}' unknown", extension),
		};

		return Ok(reader);
	}
	pub fn new_converter(
		filename: &PathBuf,
		command: &Convert,
	) -> std::io::Result<Box<dyn Converter>> {
		let extension = filename.extension().unwrap().to_str();
		let mut converter = match extension {
			Some("mbtiles") => mbtiles::Converter::new(filename).unwrap(),
			Some("cloudtiles") => cloudtiles::Converter::new(filename).unwrap(),
			Some("tar") => tar::Converter::new(filename).unwrap(),
			Some("*") => unknown::Converter::new(filename).unwrap(),
			_ => panic!("extension '{:?}' unknown", extension),
		};

		if command.precompress.is_some() {
			converter.set_precompression(command.precompress.as_ref().unwrap());
		}

		if command.min_zoom.is_some() {
			converter.set_minimum_zoom(command.min_zoom.unwrap())
		}

		if command.max_zoom.is_some() {
			converter.set_maximum_zoom(command.max_zoom.unwrap())
		}

		return Ok(converter);
	}
}