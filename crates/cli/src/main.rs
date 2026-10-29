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
	write(
		&out.join("facet-slugs.json"),
		site.slugs.to_json().to_string(),
	)?;
	write(&out.join("class-icons.css"), &site.class_icons.css)?;
	write(&out.join("class-icons.png"), &site.class_icons.png)?;
	let index = &site.search;
	let legacy = serde_json::json!({
		"names": index.names,
		"kinds": index.kinds.iter().map(|kind| kind.code()).collect::<Vec<_>>(),
		"parents": index.parents.iter().map(|parent| parent.map_or(-1, i64::from)).collect::<Vec<_>>(),
		"flags": index.flags,
		"security": index.security,
		"valueTypes": index.value_types,
		"parameterTypes": index.parameter_types,
		"typeNames": index.type_names,
		"typeEntries": index.type_entries.iter().map(|entry| entry.map_or(-1, i64::from)).collect::<Vec<_>>(),
		"updates": index.updates.iter().map(|update| serde_json::json!({"version": update.version, "hash": update.hash, "published": update.published})).collect::<Vec<_>>(),
		"removed": index.removed.iter().map(|item| {
			let mut value = serde_json::json!({"kind": item.kind.name(), "name": item.name, "version": item.version, "published": item.published});
			if let Some(parent) = &item.parent { value["parent"] = serde_json::json!(parent); }
			value
		}).collect::<Vec<_>>(),
		"classIcons": index.class_icons,
	});
	write(&out.join("search.json"), legacy.to_string())?;
	let bytes = index.encode();
	write(&out.join("search.bin"), &bytes)?;
	if let Ok(oracle) = std::env::var("SEARCH_ORACLE") {
		search_oracle(&bytes, Path::new(&oracle), &out.join("search-rust.json"))?;
	}
	Ok(())
}

fn search_oracle(bytes: &[u8], oracle: &Path, out: &Path) -> Result<(), String> {
	let engine = rapi_search::Engine::new(bytes).map_err(|error| error.to_string())?;
	let oracle: serde_json::Value =
		serde_json::from_slice(&std::fs::read(oracle).map_err(|error| error.to_string())?)
			.map_err(|error| error.to_string())?;
	let inputs: Vec<&str> = oracle["queries"]
		.as_array()
		.into_iter()
		.flatten()
		.map(|item| item["query"].as_str().unwrap_or_default())
		.collect();
	for _ in 0..1 {
		for input in &inputs {
			std::hint::black_box(rapi_search::render_view(&engine, input));
		}
	}
	let start = Instant::now();
	for _ in 0..3 {
		for input in &inputs {
			std::hint::black_box(rapi_search::render_view(&engine, input));
		}
	}
	let elapsed = start.elapsed().as_secs_f64() * 1000.0 / 3.0;
	eprintln!(
		"render_view: {elapsed:.0}ms per pass, {:.3}ms/query",
		elapsed / inputs.len() as f64
	);
	let start = Instant::now();
	let mut queries = Vec::new();
	for item in oracle["queries"].as_array().into_iter().flatten() {
		let query = item["query"].as_str().unwrap_or_default();
		let view: serde_json::Value =
			serde_json::from_str(&rapi_search::render_view(&engine, query))
				.map_err(|error| format!("{query}: {error}"))?;
		let parsed = rapi_search::parse_query(query);
		let scoped = parsed.scope.is_some();
		let results = engine.search(&parsed, if scoped { 300 } else { 80 });
		let groups = rapi_search::group_results(
			&engine,
			&results,
			if scoped { 300 } else { 8 },
			engine.type_users(&parsed, 12),
		);
		let word = parsed
			.words
			.first()
			.map(|word| word.to_lowercase())
			.unwrap_or_default();
		queries.push(serde_json::json!({
			"query": query,
			"updates": view["u"].as_array().into_iter().flatten().map(|update| update[0].clone()).collect::<Vec<_>>(),
			"groups": groups.iter().map(|group| serde_json::json!({
				"title": group.title,
				"results": group.results.iter().map(|result| serde_json::json!([
					result.entry, result.score, result.scope, result.role.map(|role| match role { rapi_search::Role::Value => "Value", rapi_search::Role::Parameter => "Parameter", rapi_search::Role::Return => "Return" }),
					rapi_search::result_href(&engine, result),
					if group.type_name.is_some() { Vec::new() } else { rapi_search::match_positions(&word, engine.name(result.entry)) },
				])).collect::<Vec<_>>(),
			})).collect::<Vec<_>>(),
			"removed": rapi_search::match_removed(&engine, &parsed, 8).iter().map(|entry| serde_json::json!([entry.kind.name(), entry.name, entry.parent, entry.version])).collect::<Vec<_>>(),
			"completion": view["c"],
		}));
	}
	eprintln!(
		"Ran {} queries in {}ms.",
		queries.len(),
		start.elapsed().as_millis()
	);
	let scores: Vec<serde_json::Value> = oracle["scores"]
		.as_array()
		.into_iter()
		.flatten()
		.map(|pair| {
			let query = pair[0].as_str().unwrap_or_default();
			let name = pair[1].as_str().unwrap_or_default();
			serde_json::json!([
				query,
				name,
				rapi_search::score_name(query, name, &name.to_ascii_lowercase()),
				rapi_search::match_positions(query, name)
			])
		})
		.collect();
	write(
		out,
		serde_json::json!({"queries": queries, "scores": scores}).to_string(),
	)
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
			let out = args
				.get(1)
				.map_or_else(|| PathBuf::from("json"), PathBuf::from);
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
