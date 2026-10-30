use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use rapi_core::generate::pipeline::{Paths, generate_site};
use rapi_core::icons::{Icon, build_favicon, build_icon_sprite, read_icon};

mod wasm;

fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<(), String> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
	}
	std::fs::write(path, contents).map_err(|error| format!("{}: {error}", path.display()))
}

fn now_millis() -> i64 {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.map_or(0, |elapsed| {
			i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
		})
}

fn generate(paths: &Paths) -> Result<(), String> {
	let start = Instant::now();
	let out = paths.generated();
	let phosphor = paths
		.root
		.join("node_modules/@phosphor-icons/core/assets/regular");
	let (site, wasm) = rayon::join(
		|| generate_site(paths),
		|| wasm::build_search_wasm(&paths.root, &out),
	);
	let site = site?;
	wasm?;
	let io = |error: std::io::Error| error.to_string();
	let build = serde_json::json!({ "lastModified": rapi_core::js::utc_string(now_millis()) });
	write(&out.join("build.json"), build.to_string())?;
	write(
		&out.join("icons.svg"),
		build_icon_sprite(&phosphor).map_err(io)?,
	)?;
	write(
		&out.join("favicon.svg"),
		build_favicon(&phosphor).map_err(io)?,
	)?;
	for icon in Icon::MASKS {
		write(
			&out.join("icons").join(format!("{}.svg", icon.name())),
			read_icon(&phosphor, icon).map_err(io)?,
		)?;
	}
	write(&out.join("class-icons.css"), &site.class_icons.css)?;
	write(&out.join("class-icons.png"), &site.class_icons.png)?;
	write(&out.join("search.bin"), site.search.encode())?;
	let summary = format!(
		"Generated {} classes, {} enums, {} data types, {} search entries, and {} updates for {} {}",
		site.api.classes.len(),
		site.api.enums.len(),
		site.api.data_types.len(),
		site.search.names.len(),
		site.updates.len(),
		site.version.channel,
		site.version.version,
	);
	let cx = rapi_site::Context::new(
		site.api,
		site.updates,
		site.stats,
		site.slugs,
		String::new(),
		String::new(),
	);
	let pages =
		serde_json::to_string(&rapi_site::page_index(&cx)).map_err(|error| error.to_string())?;
	write(&out.join("pages.json"), pages)?;
	println!("{summary} in {}ms.", start.elapsed().as_millis());
	Ok(())
}

fn site_context(paths: &Paths, sprite: String) -> Result<rapi_site::Context, String> {
	let site = generate_site(paths)?;
	Ok(rapi_site::Context::new(
		site.api,
		site.updates,
		site.stats,
		site.slugs,
		sprite,
		rapi_site::context::site_origin(),
	))
}

fn render(paths: &Paths, args: &[String]) -> Result<(), String> {
	let start = Instant::now();
	let mut dist = paths.root.join("dist");
	let mut routes = Vec::new();
	let mut args = args.iter();
	while let Some(arg) = args.next() {
		if arg == "--dist" {
			dist = args
				.next()
				.map(PathBuf::from)
				.ok_or("--dist needs a directory")?;
		} else {
			routes.push(arg.clone());
		}
	}
	let manifest_file = dist.join(".vite").join("manifest.json");
	let manifest: serde_json::Value = serde_json::from_slice(
		&std::fs::read(&manifest_file)
			.map_err(|error| format!("{}: {error}", manifest_file.display()))?,
	)
	.map_err(|error| format!("{}: {error}", manifest_file.display()))?;
	let sprite = rapi_site::build::sprite_url(&manifest).ok_or_else(|| {
		format!(
			"{} does not list {}.",
			manifest_file.display(),
			rapi_site::build::SPRITE_SOURCE
		)
	})?;
	let cx = site_context(paths, sprite)?;
	let count = rapi_site::build::write_site(
		&cx,
		&dist,
		(!routes.is_empty()).then_some(routes.as_slice()),
	)?;
	println!(
		"Prerendered and minified {count} pages in {}ms.",
		start.elapsed().as_millis()
	);
	Ok(())
}

fn dev(paths: &Paths) -> Result<(), String> {
	use std::io::{BufRead, Write};

	let start = Instant::now();
	let cx = site_context(paths, rapi_site::build::DEV_SPRITE.to_owned())?;
	eprintln!("Loaded the site data in {}ms.", start.elapsed().as_millis());
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().lock();
	for line in stdin.lock().lines() {
		let line = line.map_err(|error| error.to_string())?;
		let request: serde_json::Value =
			serde_json::from_str(&line).map_err(|error| error.to_string())?;
		let path = request["path"].as_str().unwrap_or("/");
		let template = request["template"].as_str().unwrap_or_default();
		let page = rapi_site::render_page(&cx, path);
		let html = rapi_site::template::fill_template(&cx, template, &page, path);
		let response = serde_json::json!({ "status": page.status, "html": html });
		writeln!(stdout, "{response}").map_err(|error| error.to_string())?;
		stdout.flush().map_err(|error| error.to_string())?;
	}
	Ok(())
}

const USAGE: &str = "Usage: cargo rapi <command>

Commands:
  dump-update          Move to the newest LIVE Studio build, and record its diff
  backfill             Prepend the archived builds before the first tracked version, and their diffs
  dump                 Download the API files of the current build
  diff <from> <to>     Rewrite the diff between two tracked versions
  gen                  Write src/generated: the search index and engine, icons, and build data
  render [--dist dir] [paths...]
                       Prerender pages into Vite's output directory
  dev                  Serve pages to the Vite dev server over stdin and stdout";

fn project_root() -> Result<PathBuf, String> {
	let current = std::env::current_dir().map_err(|error| error.to_string())?;
	current
		.ancestors()
		.find(|dir| dir.join("roblox-versions.json").is_file())
		.map(Path::to_path_buf)
		.ok_or_else(|| format!("{} is not inside the project.", current.display()))
}

fn run(args: &[String]) -> Result<(), String> {
	let paths = Paths::new(project_root()?);
	match args.first().map(String::as_str) {
		Some("gen") => generate(&paths),
		Some("render") => render(&paths, &args[1..]),
		Some("dev") => dev(&paths),
		Some("dump") => rapi_dump::download(&paths),
		Some("dump-update") => rapi_dump::update(&paths),
		Some("backfill") => rapi_dump::backfill::backfill(&paths),
		Some("diff") => match (args.get(1), args.get(2)) {
			(Some(from), Some(to)) => rapi_dump::rediff(&paths, from, to),
			_ => Err(USAGE.to_owned()),
		},
		Some("help" | "--help" | "-h") => {
			println!("{USAGE}");
			Ok(())
		}
		_ => Err(USAGE.to_owned()),
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
