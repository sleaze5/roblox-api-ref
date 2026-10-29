use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use rapi_core::generate::pipeline::{Paths, generate_site};

fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<(), String> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
	}
	std::fs::write(path, contents).map_err(|error| format!("{}: {error}", path.display()))
}

fn emit_json(paths: &Paths, out: &Path) -> Result<(), String> {
	let start = Instant::now();
	let site = generate_site(paths)?;
	eprintln!("Generated in {}ms.", start.elapsed().as_millis());
	let json = |value: &dyn erased::Json| value.to_json();
	write(&out.join("api.json"), json(&site.api))?;
	write(&out.join("updates.json"), json(&site.updates))?;
	write(&out.join("stats.json"), json(&site.stats))?;
	write(&out.join("facet-slugs.json"), site.slugs.to_json().to_string())?;
	write(&out.join("class-icons.css"), &site.class_icons.css)?;
	write(&out.join("class-icons.png"), &site.class_icons.png)?;
	Ok(())
}

mod erased {
	pub trait Json {
		fn to_json(&self) -> String;
	}

	impl<T: serde::Serialize> Json for T {
		fn to_json(&self) -> String {
			serde_json::to_string(self).unwrap_or_default()
		}
	}
}

fn run(args: &[String]) -> Result<(), String> {
	let paths = Paths::new(std::env::current_dir().map_err(|error| error.to_string())?);
	match args.first().map(String::as_str) {
		Some("emit-json") => {
			let out = args.get(1).map_or_else(|| PathBuf::from("json"), PathBuf::from);
			emit_json(&paths, &out)
		}
		_ => Err("Usage: rapi <command>".to_owned()),
	}
}

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().skip(1).collect();
	match run(&args) {
		Ok(()) => ExitCode::SUCCESS,
		Err(error) => {
			eprintln!("{error}");
			ExitCode::FAILURE
		}
	}
}
