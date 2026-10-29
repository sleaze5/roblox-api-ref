use wasm_bindgen::prelude::*;

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

	pub fn view(&self, query: &str) -> String {
		rapi_search::render_view(&self.0, query)
	}

	#[wasm_bindgen(js_name = latestUpload)]
	pub fn latest_upload(&self) -> Option<String> {
		self.0.latest_upload()
	}
}

#[wasm_bindgen]
pub struct Matcher(rapi_search::Matcher);

#[wasm_bindgen]
impl Matcher {
	#[wasm_bindgen(constructor)]
	pub fn new(names: &str) -> Self {
		Self(rapi_search::Matcher::new(names))
	}

	pub fn matches(&self, query: &str) -> Vec<u8> {
		self.0.matches(query)
	}
}
