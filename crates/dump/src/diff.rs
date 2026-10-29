//! The changes between two API dumps, and between their descriptions.
//!
//! Dumps stay raw JSON so that each change keeps the dump's own fields and
//! their order.

use indexmap::{IndexMap, IndexSet};
use rapi_core::dump::{
	ApiDocs, ChangeElement, ChangeType, DescriptionChange, DumpChange, FieldChange,
};
use rapi_core::model::DescriptionPart;
use serde_json::{Map, Value};

const UNORDERED_FIELDS: [&str; 2] = ["Tags", "LegacyNames"];

fn name(item: &Value) -> &str {
	item["Name"].as_str().unwrap_or_default()
}

fn list<'a>(item: &'a Value, field: &str) -> &'a [Value] {
	item[field].as_array().map_or(&[], Vec::as_slice)
}

/// A value with its object keys sorted, so that key order does not count.
fn canonical(value: &Value) -> Value {
	match value {
		Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
		Value::Object(fields) => {
			let mut keys: Vec<&String> = fields.keys().collect();
			keys.sort();
			Value::Object(
				keys.into_iter()
					.map(|key| (key.clone(), canonical(&fields[key])))
					.collect(),
			)
		}
		other => other.clone(),
	}
}

fn field_key(field: &str, value: &Value) -> String {
	let normalized = canonical(value);
	match normalized {
		Value::Array(items) if UNORDERED_FIELDS.contains(&field) => {
			let mut entries: Vec<String> = items.iter().map(Value::to_string).collect();
			entries.sort();
			serde_json::to_string(&entries).unwrap_or_default()
		}
		other => other.to_string(),
	}
}

fn compare_fields(
	before: &Map<String, Value>,
	after: &Map<String, Value>,
	ignored: &[&str],
) -> Vec<FieldChange> {
	let fields: IndexSet<&String> = before.keys().chain(after.keys()).collect();
	fields
		.into_iter()
		.filter(|field| !ignored.contains(&field.as_str()))
		.filter_map(|field| {
			let previous = before.get(field).cloned().unwrap_or(Value::Null);
			let next = after.get(field).cloned().unwrap_or(Value::Null);
			(field_key(field, &previous) != field_key(field, &next)).then(|| FieldChange {
				field: field.clone(),
				before: previous,
				after: next,
			})
		})
		.collect()
}

fn object(value: &Value) -> Map<String, Value> {
	value.as_object().cloned().unwrap_or_default()
}

/// The item without its list of children.
fn without(value: &Value, field: &str) -> Value {
	let mut fields = object(value);
	fields.shift_remove(field);
	Value::Object(fields)
}

/// Pairs items by name: every item before, then the items that are new.
fn pairs<'a>(
	before: &'a [Value],
	after: &'a [Value],
) -> Vec<(Option<&'a Value>, Option<&'a Value>)> {
	let next: IndexMap<&str, &Value> = after.iter().map(|item| (name(item), item)).collect();
	let previous: IndexMap<&str, &Value> = before.iter().map(|item| (name(item), item)).collect();
	before
		.iter()
		.map(|item| (Some(item), next.get(name(item)).copied()))
		.chain(
			after
				.iter()
				.filter(|item| !previous.contains_key(name(item)))
				.map(|item| (None, Some(item))),
		)
		.collect()
}

#[derive(Clone, Copy)]
enum Level {
	Class,
	Enum,
}

impl Level {
	const fn owner(self) -> ChangeElement {
		match self {
			Self::Class => ChangeElement::Class,
			Self::Enum => ChangeElement::Enum,
		}
	}

	const fn child(self) -> ChangeElement {
		match self {
			Self::Class => ChangeElement::Member,
			Self::Enum => ChangeElement::EnumItem,
		}
	}

	const fn children(self) -> &'static str {
		match self {
			Self::Class => "Members",
			Self::Enum => "Items",
		}
	}
}

fn change(
	element: ChangeElement,
	owner: &str,
	child: Option<&str>,
	change: ChangeType,
	changes: Option<Vec<FieldChange>>,
	value: Value,
) -> DumpChange {
	let (class, enumeration, member, item) = match element {
		ChangeElement::Class => (Some(owner), None, None, None),
		ChangeElement::Member => (Some(owner), None, child, None),
		ChangeElement::Enum => (None, Some(owner), None, None),
		ChangeElement::EnumItem => (None, Some(owner), None, child),
	};
	DumpChange {
		element,
		class: class.map(str::to_owned),
		enumeration: enumeration.map(str::to_owned),
		member: member.map(str::to_owned),
		item: item.map(str::to_owned),
		change,
		changes,
		value,
	}
}

fn diff_child(
	level: Level,
	owner: &str,
	before: Option<&Value>,
	after: Option<&Value>,
) -> Vec<DumpChange> {
	let element = level.child();
	match (before, after) {
		(Some(before), Some(after)) => {
			if matches!(level, Level::Class) && before["MemberType"] != after["MemberType"] {
				let mut changes = diff_child(level, owner, Some(before), None);
				changes.extend(diff_child(level, owner, None, Some(after)));
				return changes;
			}
			let changes = compare_fields(&object(before), &object(after), &["Name"]);
			if changes.is_empty() {
				Vec::new()
			} else {
				vec![change(
					element,
					owner,
					Some(name(after)),
					ChangeType::Change,
					Some(changes),
					after.clone(),
				)]
			}
		}
		(None, Some(after)) => vec![change(
			element,
			owner,
			Some(name(after)),
			ChangeType::Add,
			None,
			after.clone(),
		)],
		(Some(before), None) => vec![change(
			element,
			owner,
			Some(name(before)),
			ChangeType::Remove,
			None,
			before.clone(),
		)],
		(None, None) => Vec::new(),
	}
}

fn diff_owner(level: Level, before: Option<&Value>, after: Option<&Value>) -> Vec<DumpChange> {
	let children = level.children();
	match (before, after) {
		(None, Some(after)) => {
			let mut changes = vec![change(
				level.owner(),
				name(after),
				None,
				ChangeType::Add,
				None,
				without(after, children),
			)];
			changes.extend(
				list(after, children)
					.iter()
					.flat_map(|child| diff_child(level, name(after), None, Some(child))),
			);
			changes
		}
		(Some(before), None) => vec![change(
			level.owner(),
			name(before),
			None,
			ChangeType::Remove,
			None,
			without(before, children),
		)],
		(Some(before), Some(after)) => {
			let (old, new) = (without(before, children), without(after, children));
			let fields = compare_fields(&object(&old), &object(&new), &["Name"]);
			let mut changes = Vec::new();
			if !fields.is_empty() {
				changes.push(change(
					level.owner(),
					name(after),
					None,
					ChangeType::Change,
					Some(fields),
					new,
				));
			}
			for (previous, next) in pairs(list(before, children), list(after, children)) {
				changes.extend(diff_child(level, name(after), previous, next));
			}
			changes
		}
		(None, None) => Vec::new(),
	}
}

/// Every class, member, enum, and enum item that `after` adds, removes, or changes.
pub fn diff_dump(before: &Value, after: &Value) -> Vec<DumpChange> {
	let mut changes = Vec::new();
	for (previous, next) in pairs(list(before, "Classes"), list(after, "Classes")) {
		changes.extend(diff_owner(Level::Class, previous, next));
	}
	for (previous, next) in pairs(list(before, "Enums"), list(after, "Enums")) {
		changes.extend(diff_owner(Level::Enum, previous, next));
	}
	changes
}

#[derive(Clone)]
struct Target {
	element: ChangeElement,
	owner: String,
	child: Option<String>,
	value: Option<Value>,
}

struct Description {
	target: Target,
	part: DescriptionPart,
	parameter: Option<String>,
	html: Option<String>,
}

fn documentation(docs: &ApiDocs, key: Option<&str>) -> Option<String> {
	key.and_then(|key| docs.get(key))
		.and_then(|entry| entry.documentation.clone())
		.filter(|text| !text.is_empty())
}

#[derive(Default)]
struct Descriptions(IndexMap<String, Description>);

impl Descriptions {
	fn add(
		&mut self,
		target: &Target,
		part: DescriptionPart,
		html: Option<String>,
		parameter: Option<&str>,
	) {
		let kind = match target.element {
			ChangeElement::Class => "Class",
			ChangeElement::Member => "Member",
			ChangeElement::Enum => "Enum",
			ChangeElement::EnumItem => "EnumItem",
		};
		let child = target
			.child
			.as_deref()
			.map_or_else(String::new, |child| format!(".{child}"));
		let part_name = match part {
			DescriptionPart::Description => "description",
			DescriptionPart::Parameter => "parameter",
			DescriptionPart::Return => "return",
		};
		let key = format!(
			"{kind}:{}{child}/{part_name}/{}",
			target.owner,
			parameter.unwrap_or_default()
		);
		self.0.insert(
			key,
			Description {
				target: target.clone(),
				part,
				parameter: parameter.map(str::to_owned),
				html,
			},
		);
	}

	fn collect(dump: &Value, docs: &ApiDocs) -> Self {
		let mut descriptions = Self::default();
		for class in list(dump, "Classes") {
			let class_name = name(class);
			let target = Target {
				element: ChangeElement::Class,
				owner: class_name.to_owned(),
				child: None,
				value: None,
			};
			descriptions.add(
				&target,
				DescriptionPart::Description,
				documentation(docs, Some(&format!("@roblox/globaltype/{class_name}"))),
				None,
			);
			for member in list(class, "Members") {
				let key = format!("@roblox/globaltype/{class_name}.{}", name(member));
				let target = Target {
					element: ChangeElement::Member,
					owner: class_name.to_owned(),
					child: Some(name(member).to_owned()),
					value: Some(member.clone()),
				};
				descriptions.add(
					&target,
					DescriptionPart::Description,
					documentation(docs, Some(&key)),
					None,
				);
				if !matches!(member["MemberType"].as_str(), Some("Function" | "Callback")) {
					continue;
				}
				let entry = docs.get(&key);
				let parameter_keys: IndexMap<&str, &str> = entry
					.and_then(|entry| entry.params.as_ref())
					.into_iter()
					.flatten()
					.map(|parameter| (parameter.name.as_str(), parameter.documentation.as_str()))
					.collect();
				for parameter in list(member, "Parameters") {
					let parameter_name = name(parameter);
					let html = documentation(docs, parameter_keys.get(parameter_name).copied());
					descriptions.add(
						&target,
						DescriptionPart::Parameter,
						html,
						Some(parameter_name),
					);
				}
				let returns = entry
					.and_then(|entry| entry.returns.as_ref())
					.and_then(|returns| returns.first());
				descriptions.add(
					&target,
					DescriptionPart::Return,
					documentation(docs, returns.map(String::as_str)),
					None,
				);
			}
		}
		for enumeration in list(dump, "Enums") {
			let enum_name = name(enumeration);
			let target = Target {
				element: ChangeElement::Enum,
				owner: enum_name.to_owned(),
				child: None,
				value: None,
			};
			descriptions.add(
				&target,
				DescriptionPart::Description,
				documentation(docs, Some(&format!("@roblox/global/Enum.{enum_name}"))),
				None,
			);
			for item in list(enumeration, "Items") {
				let target = Target {
					element: ChangeElement::EnumItem,
					owner: enum_name.to_owned(),
					child: Some(name(item).to_owned()),
					value: Some(item.clone()),
				};
				let key = format!("@roblox/enum/{enum_name}.{}", name(item));
				descriptions.add(
					&target,
					DescriptionPart::Description,
					documentation(docs, Some(&key)),
					None,
				);
			}
		}
		descriptions
	}
}

/// Every description that changed between two documented dumps, for the items in `after`.
pub fn diff_descriptions(
	before: (&Value, &ApiDocs),
	after: (&Value, &ApiDocs),
) -> Vec<DescriptionChange> {
	let previous = Descriptions::collect(before.0, before.1);
	Descriptions::collect(after.0, after.1)
		.0
		.into_iter()
		.filter_map(|(key, description)| {
			let old = previous.0.get(&key).and_then(|old| old.html.clone());
			(old != description.html).then(|| {
				let target = description.target;
				let (class, enumeration, member, item) = match target.element {
					ChangeElement::Class => (Some(target.owner), None, None, None),
					ChangeElement::Member => (Some(target.owner), None, target.child, None),
					ChangeElement::Enum => (None, Some(target.owner), None, None),
					ChangeElement::EnumItem => (None, Some(target.owner), None, target.child),
				};
				DescriptionChange {
					element: target.element,
					class,
					enumeration,
					member,
					item,
					part: description.part,
					parameter: description.parameter,
					before: old,
					after: description.html,
					value: target.value,
				}
			})
		})
		.collect()
}

#[cfg(test)]
#[expect(
	clippy::needless_pass_by_value,
	reason = "the helpers take dumps the way they read"
)]
mod tests {
	use serde_json::json;

	use super::*;

	fn property(name: &str, extra: &Value) -> Value {
		let mut value = json!({
			"MemberType": "Property",
			"Name": name,
			"Category": "Data",
			"ValueType": { "Category": "Primitive", "Name": "bool" },
			"Security": { "Read": "None", "Write": "None" },
			"Serialization": { "CanLoad": true, "CanSave": true },
			"ThreadSafety": "ReadSafe",
		});
		for (key, field) in extra.as_object().into_iter().flatten() {
			value[key] = field.clone();
		}
		value
	}

	fn method(name: &str) -> Value {
		json!({
			"MemberType": "Function",
			"Name": name,
			"Parameters": [],
			"ReturnType": { "Category": "Primitive", "Name": "null" },
			"Security": "None",
			"ThreadSafety": "Unsafe",
		})
	}

	fn class(name: &str, members: Vec<Value>, tags: Option<Value>) -> Value {
		let mut value = json!({ "Name": name, "Superclass": "Instance", "MemoryCategory": "Instances", "Members": members });
		if let Some(tags) = tags {
			value["Tags"] = tags;
		}
		value
	}

	fn dump(classes: Vec<Value>, enums: Value) -> Value {
		json!({ "Version": 1, "Classes": classes, "Enums": enums })
	}

	#[test]
	fn finds_nothing_between_identical_dumps() {
		let same = dump(
			vec![class("Part", vec![property("Anchored", &json!({}))], None)],
			json!([]),
		);
		assert!(diff_dump(&same, &same.clone()).is_empty());
	}

	#[test]
	fn adds_a_class_with_its_members_and_removes_a_class_without_them() {
		let changes = diff_dump(
			&dump(
				vec![class("Old", vec![property("Gone", &json!({}))], None)],
				json!([]),
			),
			&dump(
				vec![class("New", vec![property("Fresh", &json!({}))], None)],
				json!([]),
			),
		);
		let kinds: Vec<(ChangeType, ChangeElement)> = changes
			.iter()
			.map(|change| (change.change, change.element))
			.collect();
		assert_eq!(
			kinds,
			[
				(ChangeType::Remove, ChangeElement::Class),
				(ChangeType::Add, ChangeElement::Class),
				(ChangeType::Add, ChangeElement::Member),
			]
		);
		assert_eq!(
			(changes[2].class.as_deref(), changes[2].member.as_deref()),
			(Some("New"), Some("Fresh"))
		);
		assert!(changes[1].value.get("Members").is_none());
	}

	#[test]
	fn records_the_fields_that_changed_with_their_previous_and_new_values() {
		let changes = diff_dump(
			&dump(
				vec![class("Part", vec![property("Size", &json!({}))], None)],
				json!([]),
			),
			&dump(
				vec![class(
					"Part",
					vec![property("Size", &json!({ "Tags": ["Deprecated"] }))],
					None,
				)],
				json!([]),
			),
		);
		assert_eq!(
			serde_json::to_value(&changes).unwrap(),
			json!([{
				"element": "Member",
				"class": "Part",
				"member": "Size",
				"type": "Change",
				"changes": [{ "field": "Tags", "before": null, "after": ["Deprecated"] }],
				"value": property("Size", &json!({ "Tags": ["Deprecated"] })),
			}])
		);
	}

	#[test]
	fn ignores_the_order_of_tags_and_of_object_keys() {
		assert!(
			diff_dump(
				&dump(
					vec![class(
						"Part",
						vec![],
						Some(json!(["NotCreatable", "Service"]))
					)],
					json!([])
				),
				&dump(
					vec![class(
						"Part",
						vec![],
						Some(json!(["Service", "NotCreatable"]))
					)],
					json!([])
				),
			)
			.is_empty()
		);
		let reordered = json!({ "Write": "None", "Read": "None" });
		assert!(
			diff_dump(
				&dump(
					vec![class("Part", vec![property("Size", &json!({}))], None)],
					json!([])
				),
				&dump(
					vec![class(
						"Part",
						vec![property("Size", &json!({ "Security": reordered }))],
						None
					)],
					json!([])
				),
			)
			.is_empty()
		);
	}

	#[test]
	fn replaces_a_member_whose_kind_changed() {
		let changes = diff_dump(
			&dump(
				vec![class("Part", vec![property("Touch", &json!({}))], None)],
				json!([]),
			),
			&dump(vec![class("Part", vec![method("Touch")], None)], json!([])),
		);
		let kinds: Vec<ChangeType> = changes.iter().map(|change| change.change).collect();
		assert_eq!(kinds, [ChangeType::Remove, ChangeType::Add]);
	}

	#[test]
	fn diffs_enum_items() {
		let changes = diff_dump(
			&dump(
				vec![],
				json!([{ "Name": "Material", "Items": [{ "Name": "Grass", "Value": 1 }] }]),
			),
			&dump(
				vec![],
				json!([{ "Name": "Material", "Items": [{ "Name": "Grass", "Value": 2 }, { "Name": "Sand", "Value": 3 }] }]),
			),
		);
		assert_eq!(
			serde_json::to_value(&changes).unwrap(),
			json!([
				{
					"element": "EnumItem",
					"enum": "Material",
					"item": "Grass",
					"type": "Change",
					"changes": [{ "field": "Value", "before": 1, "after": 2 }],
					"value": { "Name": "Grass", "Value": 2 },
				},
				{ "element": "EnumItem", "enum": "Material", "item": "Sand", "type": "Add", "value": { "Name": "Sand", "Value": 3 } },
			])
		);
	}

	fn touch() -> Value {
		json!({
			"MemberType": "Function",
			"Name": "Touch",
			"Parameters": [{ "Name": "other", "Type": { "Category": "Class", "Name": "BasePart" } }],
			"ReturnType": { "Category": "Primitive", "Name": "bool" },
			"Security": "None",
		})
	}

	fn documented() -> Value {
		json!({
			"Version": 1,
			"Classes": [{ "Name": "Part", "Superclass": "Instance", "MemoryCategory": "Instances", "Members": [touch()] }],
			"Enums": [{ "Name": "Material", "Items": [{ "Name": "Grass", "Value": 1 }] }],
		})
	}

	fn docs(overrides: &Value) -> ApiDocs {
		let mut docs = json!({
			"@roblox/globaltype/Part": { "documentation": "A part." },
			"@roblox/globaltype/Part.Touch": {
				"documentation": "Touches.",
				"params": [{ "name": "other", "documentation": "@roblox/globaltype/Part.Touch/param/0" }],
				"returns": ["@roblox/globaltype/Part.Touch/return/0"],
			},
			"@roblox/globaltype/Part.Touch/param/0": { "documentation": "The other part." },
			"@roblox/globaltype/Part.Touch/return/0": { "documentation": "Whether it worked." },
			"@roblox/global/Enum.Material": { "documentation": "Materials." },
			"@roblox/enum/Material.Grass": { "documentation": "Grass." },
		});
		for (key, value) in overrides.as_object().into_iter().flatten() {
			if value.is_null() {
				docs.as_object_mut().unwrap().shift_remove(key);
			} else {
				docs[key] = value.clone();
			}
		}
		serde_json::from_value(docs).unwrap()
	}

	#[test]
	fn finds_no_description_changes_when_the_descriptions_match() {
		let dump = documented();
		assert!(
			diff_descriptions((&dump, &docs(&json!({}))), (&dump, &docs(&json!({})))).is_empty()
		);
	}

	#[test]
	fn records_added_changed_and_removed_descriptions_with_their_targets() {
		let dump = documented();
		let before = docs(&json!({ "@roblox/enum/Material.Grass": null }));
		let after = docs(&json!({
			"@roblox/globaltype/Part.Touch/param/0": { "documentation": "Another part." },
			"@roblox/global/Enum.Material": null,
		}));
		assert_eq!(
			serde_json::to_value(diff_descriptions((&dump, &before), (&dump, &after))).unwrap(),
			json!([
				{
					"element": "Member",
					"class": "Part",
					"member": "Touch",
					"part": "parameter",
					"parameter": "other",
					"before": "The other part.",
					"after": "Another part.",
					"value": touch(),
				},
				{ "element": "Enum", "enum": "Material", "part": "description", "before": "Materials.", "after": null },
				{
					"element": "EnumItem",
					"enum": "Material",
					"item": "Grass",
					"part": "description",
					"before": null,
					"after": "Grass.",
					"value": { "Name": "Grass", "Value": 1 },
				},
			])
		);
	}

	#[test]
	fn skips_the_descriptions_of_removed_members() {
		let before = documented();
		let mut after = before.clone();
		after["Classes"][0]["Members"] = json!([]);
		assert!(
			diff_descriptions((&before, &docs(&json!({}))), (&after, &docs(&json!({})))).is_empty()
		);
	}
}
