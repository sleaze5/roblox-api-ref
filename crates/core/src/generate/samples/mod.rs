mod format;
mod highlight;

use std::collections::HashMap;

use indexmap::IndexMap;

pub use self::format::format_sample;
pub use self::highlight::highlight;
use super::docs::ApiNames;
use crate::dump::ApiDocs;
use crate::model::{Class, CodeSample, DataType, Enum};

struct Samples<'a, 'b> {
	names: ApiNames<'a>,
	docs: &'b ApiDocs,
	cache: HashMap<&'b str, Option<CodeSample>>,
}

impl<'b> Samples<'_, 'b> {
	fn get(&mut self, key: &str) -> Option<CodeSample> {
		let code = self.docs.get(key)?.code_sample.as_deref()?;
		if let Some(found) = self.cache.get(code) {
			return found.clone();
		}
		let sample = format_sample(code).map(|source| CodeSample {
			tokens: highlight(&self.names, &source),
			source,
		});
		self.cache.insert(code, sample.clone());
		sample
	}
}

pub fn apply_code_samples(
	classes: &mut IndexMap<String, Class>,
	enums: &IndexMap<String, Enum>,
	data_types: &IndexMap<String, DataType>,
	docs: &ApiDocs,
) {
	let found: Vec<(Option<CodeSample>, Vec<Option<CodeSample>>)> = {
		let mut samples = Samples {
			names: ApiNames {
				classes,
				enums,
				data_types,
			},
			docs,
			cache: HashMap::new(),
		};
		classes
			.values()
			.map(|item| {
				let own = samples.get(&format!("@roblox/globaltype/{}", item.name));
				let members = item
					.members
					.iter()
					.map(|member| {
						samples.get(&format!("@roblox/globaltype/{}.{}", item.name, member.name))
					})
					.collect();
				(own, members)
			})
			.collect()
	};
	for (item, (sample, members)) in classes.values_mut().zip(found) {
		item.sample = sample;
		for (member, sample) in item.members.iter_mut().zip(members) {
			member.sample = sample;
		}
	}
}
