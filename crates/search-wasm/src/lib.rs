//! Browser bindings for the search engine and the in-page name filters.

use wasm_bindgen::prelude::*;

/// The search dialog's engine, built from the binary search index.
#[wasm_bindgen]
pub struct Search(rapi_search::Engine);

#[wasm_bindgen]
impl Search {
	#[wasm_bindgen(constructor)]
	pub fn new(index: &[u8]) -> Result<Self, JsError> {
		rapi_search::Engine::new(index)
			.map(Self)
			.map_err(|error| JsError::new(&error.to_string()))
	}

	/// Everything the dialog shows for `query`, as compact JSON.
	pub fn view(&self, query: &str) -> String {
		rapi_search::render_view(&self.0, query)
	}

	/// The upload name of the newest update, for the syntax help.
	#[wasm_bindgen(js_name = latestUpload)]
	pub fn latest_upload(&self) -> Option<String> {
		self.0.latest_upload()
	}
}

/// Matches a fixed list of names against filter text, like the search does.
#[wasm_bindgen]
pub struct Matcher(rapi_search::Matcher);

#[wasm_bindgen]
impl Matcher {
	/// `names` holds one name per line.
	#[wasm_bindgen(constructor)]
	pub fn new(names: &str) -> Self {
		Self(rapi_search::Matcher::new(names))
	}

	/// One byte per name: 1 when it matches `query`.
	pub fn matches(&self, query: &str) -> Vec<u8> {
		self.0.matches(query)
	}
}
