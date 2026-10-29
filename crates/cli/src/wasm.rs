use std::path::Path;
use std::process::Command;

use wasm_bindgen_cli_support::Bindgen;

const CRATE: &str = "rapi-search-wasm";
const TARGET: &str = "wasm32-unknown-unknown";
const PROFILE: &str = "wasm";

pub fn build_search_wasm(root: &Path, out_dir: &Path) -> Result<(), String> {
	let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
	let status = Command::new(cargo)
		.current_dir(root)
		.args([
			"build",
			"--quiet",
			"--package",
			CRATE,
			"--target",
			TARGET,
			"--profile",
			PROFILE,
		])
		.status()
		.map_err(|error| format!("Could not run Cargo: {error}"))?;
	if !status.success() {
		return Err(format!("Building {CRATE} failed with {status}."));
	}
	let target =
		std::env::var_os("CARGO_TARGET_DIR").map_or_else(|| root.join("target"), Into::into);
	let module = target
		.join(TARGET)
		.join(PROFILE)
		.join(format!("{}.wasm", CRATE.replace('-', "_")));
	Bindgen::new()
		.input_path(module)
		.out_name("search")
		.web(true)
		.map_err(|error| error.to_string())?
		.typescript(true)
		.omit_default_module_path(true)
		.generate(out_dir)
		.map_err(|error| format!("wasm-bindgen failed: {error}"))
}
