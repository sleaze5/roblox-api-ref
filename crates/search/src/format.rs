const MAGIC: &[u8; 8] = b"RAPISRC3";
pub const USAGE_LIMIT: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
	Value,
	Parameter,
	Return,
}

impl Role {
	pub const fn label(self) -> &'static str {
		match self {
			Self::Value => "Property type",
			Self::Parameter => "Parameter",
			Self::Return => "Return value",
		}
	}

	pub const fn code(self) -> u8 {
		self as u8
	}

	pub const fn from_code(code: u8) -> Self {
		match code {
			0 => Self::Value,
			1 => Self::Parameter,
			_ => Self::Return,
		}
	}
}

pub mod flags {
	pub const DEPRECATED: u16 = 1;
	pub const HIDDEN: u16 = 2;
	pub const NOT_SCRIPTABLE: u16 = 4;
	pub const SERVICE: u16 = 8;
	pub const READ_ONLY: u16 = 16;
	pub const YIELDS: u16 = 32;
	pub const NOT_CREATABLE: u16 = 64;
	pub const NOT_REPLICATED: u16 = 128;
	pub const INTERNAL: u16 = 256;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryKind {
	Class,
	Enum,
	Property,
	Method,
	Event,
	Callback,
	EnumItem,
	DataType,
}

impl EntryKind {
	pub const ALL: [Self; 8] = [
		Self::Class,
		Self::Enum,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::EnumItem,
		Self::DataType,
	];

	pub fn from_code(code: u8) -> Self {
		Self::ALL
			.get(usize::from(code))
			.copied()
			.unwrap_or(Self::Class)
	}

	pub const fn code(self) -> u8 {
		self as u8
	}

	pub const fn name(self) -> &'static str {
		match self {
			Self::Class => "Class",
			Self::Enum => "Enum",
			Self::Property => "Property",
			Self::Method => "Method",
			Self::Event => "Event",
			Self::Callback => "Callback",
			Self::EnumItem => "EnumItem",
			Self::DataType => "DataType",
		}
	}

	pub const fn is_primary(self) -> bool {
		matches!(self, Self::Class | Self::Enum | Self::DataType)
	}

	pub const fn order(self) -> u8 {
		match self {
			Self::Class => 0,
			Self::DataType => 1,
			Self::Enum => 2,
			Self::Property => 3,
			Self::Method => 4,
			Self::Event => 5,
			Self::Callback => 6,
			Self::EnumItem => 7,
		}
	}

	pub const fn boost(self) -> i64 {
		match self {
			Self::Class => 40,
			Self::Enum | Self::DataType => 30,
			Self::Property | Self::Method | Self::Event => 10,
			Self::Callback => 5,
			Self::EnumItem => 0,
		}
	}

	pub const fn member(self) -> Option<rapi_routes::MemberKind> {
		match self {
			Self::Property => Some(rapi_routes::MemberKind::Property),
			Self::Method => Some(rapi_routes::MemberKind::Method),
			Self::Event => Some(rapi_routes::MemberKind::Event),
			Self::Callback => Some(rapi_routes::MemberKind::Callback),
			_ => None,
		}
	}
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UpdateData {
	pub version: String,
	pub hash: String,
	pub published: String,
	pub date: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedData {
	pub kind: EntryKind,
	pub name: String,
	pub parent: Option<String>,
	pub version: String,
	pub published: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IndexData {
	pub names: Vec<String>,
	pub kinds: Vec<EntryKind>,
	pub parents: Vec<Option<u32>>,
	pub flags: Vec<u16>,
	pub security: Vec<u8>,
	pub users: Vec<Vec<(u32, Role)>>,
	pub value_types: Vec<Vec<u32>>,
	pub parameter_types: Vec<Vec<u32>>,
	pub type_names: Vec<String>,
	pub type_entries: Vec<Option<u32>>,
	pub class_icons: Vec<Option<u32>>,
	pub updates: Vec<UpdateData>,
	pub removed: Vec<RemovedData>,
}

const NO_ID: u16 = u16::MAX;

struct Writer(Vec<u8>);

fn id16(value: Option<u32>) -> u16 {
	value.map_or(NO_ID, |value| {
		u16::try_from(value)
			.ok()
			.filter(|value| *value != NO_ID)
			.expect("the search index holds fewer than 65535 entries and types")
	})
}

impl Writer {
	fn count(&mut self, count: usize) {
		let count = u32::try_from(count).expect("index sections fit in 32 bits");
		self.0.extend_from_slice(&count.to_le_bytes());
	}

	fn u8s(&mut self, values: &[u8]) {
		self.count(values.len());
		self.0.extend_from_slice(values);
	}

	fn u16s(&mut self, values: impl ExactSizeIterator<Item = u16>) {
		self.count(values.len());
		for value in values {
			self.0.extend_from_slice(&value.to_le_bytes());
		}
	}

	fn strings<'a>(&mut self, values: impl ExactSizeIterator<Item = &'a str>) {
		let count = values.len();
		let joined: Vec<&str> = values
			.inspect(|value| assert!(!value.contains('\0'), "index strings have no NUL"))
			.collect();
		self.count(count);
		self.u8s(joined.join("\0").as_bytes());
	}

	fn lists(&mut self, lists: &[Vec<u32>]) {
		let counts: Vec<u8> = lists
			.iter()
			.map(|list| u8::try_from(list.len()).expect("an entry uses fewer than 256 types"))
			.collect();
		self.u8s(&counts);
		self.u16s(
			lists
				.iter()
				.flatten()
				.map(|id| id16(Some(*id)))
				.collect::<Vec<_>>()
				.into_iter(),
		);
	}
}

impl IndexData {
	fn role_of(&self, entry: usize, id: u32) -> Role {
		if !self.value_types[entry].contains(&id) {
			return Role::Parameter;
		}
		if self.kinds[entry] == EntryKind::Property {
			Role::Value
		} else {
			Role::Return
		}
	}

	pub fn link_users(&mut self, ranks: &[u32]) {
		let rank = |entry: Option<u32>| entry.map_or(0, |entry| ranks[entry as usize]);
		let key = |entry: usize| {
			(
				self.flags[entry] & flags::DEPRECATED,
				self.kinds[entry].order(),
				rank(self.parents[entry]),
				ranks[entry],
			)
		};
		let mut users: Vec<Vec<usize>> = vec![Vec::new(); self.type_names.len()];
		for entry in 0..self.names.len() {
			let mut seen: Vec<u32> = Vec::new();
			for id in self.value_types[entry]
				.iter()
				.chain(&self.parameter_types[entry])
			{
				if !seen.contains(id) {
					seen.push(*id);
					users[*id as usize].push(entry);
				}
			}
		}
		self.users = users
			.into_iter()
			.enumerate()
			.map(|(id, mut list)| {
				if self.type_entries[id].is_none() {
					return Vec::new();
				}
				list.sort_by_key(|entry| key(*entry));
				list.truncate(USAGE_LIMIT);
				let id = u32::try_from(id).expect("types fit in 32 bits");
				list.into_iter()
					.map(|entry| {
						(
							u32::try_from(entry).expect("entries fit in 32 bits"),
							self.role_of(entry, id),
						)
					})
					.collect()
			})
			.collect();
	}

	pub fn encode(&self) -> Vec<u8> {
		let mut writer = Writer(MAGIC.to_vec());
		writer.strings(self.names.iter().map(String::as_str));
		writer.u8s(
			&self
				.kinds
				.iter()
				.map(|kind| kind.code())
				.collect::<Vec<_>>(),
		);
		writer.u16s(self.parents.iter().map(|parent| id16(*parent)));
		writer.u16s(self.flags.iter().copied());
		writer.u8s(&self.security);
		writer.lists(&self.value_types);
		writer.lists(&self.parameter_types);
		writer.strings(self.type_names.iter().map(String::as_str));
		writer.lists(
			&self
				.users
				.iter()
				.map(|users| users.iter().map(|(entry, _)| *entry).collect())
				.collect::<Vec<_>>(),
		);
		writer.u8s(
			&self
				.users
				.iter()
				.flatten()
				.map(|(_, role)| role.code())
				.collect::<Vec<_>>(),
		);
		writer.u16s(self.type_entries.iter().map(|entry| id16(*entry)));
		writer.u16s(self.class_icons.iter().map(|icon| id16(*icon)));
		writer.strings(self.updates.iter().map(|update| update.version.as_str()));
		writer.strings(self.updates.iter().map(|update| update.hash.as_str()));
		writer.strings(self.updates.iter().map(|update| update.published.as_str()));
		writer.strings(self.updates.iter().map(|update| update.date.as_str()));
		writer.u8s(
			&self
				.removed
				.iter()
				.map(|item| item.kind.code())
				.collect::<Vec<_>>(),
		);
		writer.strings(self.removed.iter().map(|item| item.name.as_str()));
		writer.strings(
			self.removed
				.iter()
				.map(|item| item.parent.as_deref().unwrap_or_default()),
		);
		writer.u16s(self.removed.iter().map(|item| {
			let update = self
				.updates
				.iter()
				.position(|update| {
					update.version == item.version && update.published == item.published
				})
				.expect("removed items belong to listed updates");
			u16::try_from(update).expect("updates fit in 16 bits")
		}));
		writer.0
	}

	pub fn decode(bytes: &[u8]) -> Result<Self, IndexError> {
		let index = Index::new(bytes)?;
		let count = index.len();
		let types = index.type_count();
		Ok(Self {
			names: (0..count)
				.map(|entry| index.name(entry).to_owned())
				.collect(),
			kinds: (0..count).map(|entry| index.kind(entry)).collect(),
			parents: (0..count).map(|entry| index.parent(entry)).collect(),
			flags: (0..count).map(|entry| index.flags(entry)).collect(),
			security: (0..count).map(|entry| index.security(entry)).collect(),
			value_types: (0..count)
				.map(|entry| index.value_types(entry).collect())
				.collect(),
			parameter_types: (0..count)
				.map(|entry| index.parameter_types(entry).collect())
				.collect(),
			type_names: (0..types)
				.map(|id| index.type_name(id).to_owned())
				.collect(),
			users: (0..types).map(|id| index.users(id).collect()).collect(),
			type_entries: (0..types).map(|id| index.type_entry(id)).collect(),
			class_icons: (0..index.class_icons.len())
				.map(|entry| index.class_icon(entry))
				.collect(),
			updates: (0..index.update_count())
				.map(|position| index.update(position))
				.collect(),
			removed: (0..index.removed_count())
				.map(|position| index.removed(position))
				.collect(),
		})
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexError(pub String);

impl std::fmt::Display for IndexError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str(&self.0)
	}
}

impl std::error::Error for IndexError {}

fn invalid() -> IndexError {
	IndexError("The search index is invalid.".to_owned())
}

struct Reader<'a> {
	bytes: &'a [u8],
	at: usize,
}

impl<'a> Reader<'a> {
	fn count(&mut self) -> Result<usize, IndexError> {
		let word = self.bytes.get(self.at..self.at + 4).ok_or_else(invalid)?;
		self.at += 4;
		Ok(u32::from_le_bytes([word[0], word[1], word[2], word[3]]) as usize)
	}

	fn take(&mut self, length: usize) -> Result<&'a [u8], IndexError> {
		let end = self.at.checked_add(length).ok_or_else(invalid)?;
		let slice = self.bytes.get(self.at..end).ok_or_else(invalid)?;
		self.at = end;
		Ok(slice)
	}

	fn u8s(&mut self) -> Result<Vec<u8>, IndexError> {
		let count = self.count()?;
		Ok(self.take(count)?.to_vec())
	}

	fn u16s(&mut self) -> Result<Vec<u16>, IndexError> {
		let count = self.count()?;
		Ok(self
			.take(count.checked_mul(2).ok_or_else(invalid)?)?
			.chunks_exact(2)
			.map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
			.collect())
	}

	fn strings(&mut self) -> Result<Strings, IndexError> {
		let count = self.count()?;
		let bytes = self.u8s()?;
		let text = String::from_utf8(bytes).map_err(|_| invalid())?;
		let mut offsets = Vec::with_capacity(count + 1);
		offsets.push(0);
		if count > 0 {
			offsets.extend(
				text.match_indices('\0')
					.map(|(position, _)| u32::try_from(position + 1).unwrap_or(u32::MAX)),
			);
			offsets.push(u32::try_from(text.len() + 1).map_err(|_| invalid())?);
		}
		if offsets.len() != count + 1 {
			return Err(invalid());
		}
		Ok(Strings { text, offsets })
	}

	fn lists(&mut self) -> Result<Lists, IndexError> {
		let counts = self.u8s()?;
		let ids = self.u16s()?;
		let mut offsets = Vec::with_capacity(counts.len() + 1);
		let mut total = 0;
		offsets.push(0);
		for count in &counts {
			total += u32::from(*count);
			offsets.push(total);
		}
		if total as usize != ids.len() {
			return Err(invalid());
		}
		Ok(Lists { offsets, ids })
	}
}

struct Strings {
	text: String,
	offsets: Vec<u32>,
}

impl Strings {
	const fn len(&self) -> usize {
		self.offsets.len() - 1
	}

	fn get(&self, position: usize) -> &str {
		let start = self.offsets[position] as usize;
		let end = self.offsets[position + 1] as usize - 1;
		&self.text[start..end]
	}
}

struct Lists {
	offsets: Vec<u32>,
	ids: Vec<u16>,
}

impl Lists {
	fn get(&self, entry: usize) -> &[u16] {
		&self.ids[self.offsets[entry] as usize..self.offsets[entry + 1] as usize]
	}
}

fn optional(value: u16) -> Option<u32> {
	(value != NO_ID).then_some(u32::from(value))
}

pub struct Index {
	names: Strings,
	kinds: Vec<u8>,
	parents: Vec<u16>,
	flags: Vec<u16>,
	security: Vec<u8>,
	value_types: Lists,
	parameter_types: Lists,
	type_names: Strings,
	users: Lists,
	user_roles: Vec<u8>,
	type_entries: Vec<u16>,
	class_icons: Vec<u16>,
	update_versions: Strings,
	update_hashes: Strings,
	update_published: Strings,
	update_dates: Strings,
	removed_kinds: Vec<u8>,
	removed_names: Strings,
	removed_parents: Strings,
	removed_updates: Vec<u16>,
}

pub fn decode_check(bytes: &[u8]) -> Result<(), IndexError> {
	Index::new(bytes).map(|_| ())
}

impl Index {
	pub fn new(bytes: &[u8]) -> Result<Self, IndexError> {
		if !bytes.starts_with(MAGIC) {
			return Err(IndexError("This is not a search index.".to_owned()));
		}
		let mut reader = Reader {
			bytes,
			at: MAGIC.len(),
		};
		let index = Self {
			names: reader.strings()?,
			kinds: reader.u8s()?,
			parents: reader.u16s()?,
			flags: reader.u16s()?,
			security: reader.u8s()?,
			value_types: reader.lists()?,
			parameter_types: reader.lists()?,
			type_names: reader.strings()?,
			users: reader.lists()?,
			user_roles: reader.u8s()?,
			type_entries: reader.u16s()?,
			class_icons: reader.u16s()?,
			update_versions: reader.strings()?,
			update_hashes: reader.strings()?,
			update_published: reader.strings()?,
			update_dates: reader.strings()?,
			removed_kinds: reader.u8s()?,
			removed_names: reader.strings()?,
			removed_parents: reader.strings()?,
			removed_updates: reader.u16s()?,
		};
		if reader.at != bytes.len() || !index.is_consistent() {
			return Err(invalid());
		}
		Ok(index)
	}

	fn is_consistent(&self) -> bool {
		let entries = self.len();
		let types = self.type_count();
		let in_range = |value: u16, limit: usize| value == NO_ID || usize::from(value) < limit;
		[
			self.kinds.len(),
			self.parents.len(),
			self.flags.len(),
			self.security.len(),
			self.value_types.offsets.len() - 1,
			self.parameter_types.offsets.len() - 1,
		]
		.iter()
		.all(|count| *count == entries)
			&& self.type_entries.len() == types
			&& self.users.offsets.len() - 1 == types
			&& self.user_roles.len() == self.users.ids.len()
			&& self.user_roles.iter().all(|role| *role < 3)
			&& self
				.users
				.ids
				.iter()
				.all(|entry| usize::from(*entry) < entries)
			&& self.class_icons.len() <= entries
			&& self.parents.iter().all(|parent| in_range(*parent, entries))
			&& self
				.type_entries
				.iter()
				.all(|entry| in_range(*entry, entries))
			&& self
				.value_types
				.ids
				.iter()
				.all(|id| usize::from(*id) < types)
			&& self
				.parameter_types
				.ids
				.iter()
				.all(|id| usize::from(*id) < types)
			&& [
				&self.update_hashes,
				&self.update_published,
				&self.update_dates,
			]
			.iter()
			.all(|table| table.len() == self.update_versions.len())
			&& self.removed_kinds.len() == self.removed_names.len()
			&& self.removed_parents.len() == self.removed_names.len()
			&& self.removed_updates.len() == self.removed_names.len()
			&& self
				.removed_updates
				.iter()
				.all(|update| usize::from(*update) < self.update_versions.len())
	}

	pub const fn len(&self) -> usize {
		self.names.len()
	}

	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn name(&self, entry: usize) -> &str {
		self.names.get(entry)
	}

	pub fn names_text(&self) -> (&str, &[u32]) {
		(&self.names.text, &self.names.offsets)
	}

	pub fn kind(&self, entry: usize) -> EntryKind {
		EntryKind::from_code(self.kinds[entry])
	}

	pub fn parent(&self, entry: usize) -> Option<u32> {
		optional(self.parents[entry])
	}

	pub fn flags(&self, entry: usize) -> u16 {
		self.flags[entry]
	}

	pub fn security(&self, entry: usize) -> u8 {
		self.security[entry]
	}

	pub fn users(&self, id: usize) -> impl Iterator<Item = (u32, Role)> + '_ {
		let start = self.users.offsets[id] as usize;
		self.users
			.get(id)
			.iter()
			.zip(&self.user_roles[start..])
			.map(|(entry, role)| (u32::from(*entry), Role::from_code(*role)))
	}

	pub fn value_types(&self, entry: usize) -> impl Iterator<Item = u32> + '_ {
		self.value_types.get(entry).iter().map(|id| u32::from(*id))
	}

	pub fn parameter_types(&self, entry: usize) -> impl Iterator<Item = u32> + '_ {
		self.parameter_types
			.get(entry)
			.iter()
			.map(|id| u32::from(*id))
	}

	pub const fn type_count(&self) -> usize {
		self.type_names.len()
	}

	pub fn type_name(&self, id: usize) -> &str {
		self.type_names.get(id)
	}

	pub fn type_entry(&self, id: usize) -> Option<u32> {
		optional(self.type_entries[id])
	}

	pub fn class_icon(&self, entry: usize) -> Option<u32> {
		self.class_icons.get(entry).copied().and_then(optional)
	}

	pub const fn update_count(&self) -> usize {
		self.update_versions.len()
	}

	pub fn update(&self, position: usize) -> UpdateData {
		UpdateData {
			version: self.update_versions.get(position).to_owned(),
			hash: self.update_hashes.get(position).to_owned(),
			published: self.update_published.get(position).to_owned(),
			date: self.update_dates.get(position).to_owned(),
		}
	}

	pub fn update_version(&self, position: usize) -> &str {
		self.update_versions.get(position)
	}

	pub fn update_hash(&self, position: usize) -> &str {
		self.update_hashes.get(position)
	}

	pub fn update_date(&self, position: usize) -> &str {
		self.update_dates.get(position)
	}

	pub const fn removed_count(&self) -> usize {
		self.removed_names.len()
	}

	pub fn removed(&self, position: usize) -> RemovedData {
		let parent = self.removed_parents.get(position);
		let update = usize::from(self.removed_updates[position]);
		RemovedData {
			kind: EntryKind::from_code(self.removed_kinds[position]),
			name: self.removed_names.get(position).to_owned(),
			parent: (!parent.is_empty()).then(|| parent.to_owned()),
			version: self.update_versions.get(update).to_owned(),
			published: self.update_published.get(update).to_owned(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	pub fn sample() -> IndexData {
		IndexData {
			names: vec![
				"Part".into(),
				"Anchored".into(),
				"Material".into(),
				"Plastic".into(),
			],
			kinds: vec![
				EntryKind::Class,
				EntryKind::Property,
				EntryKind::Enum,
				EntryKind::EnumItem,
			],
			parents: vec![None, Some(0), None, Some(2)],
			flags: vec![0, flags::DEPRECATED, 0, 0],
			security: vec![0, 1, 0, 0],
			users: vec![vec![(1, Role::Value)]],
			value_types: vec![vec![], vec![0], vec![], vec![]],
			parameter_types: vec![vec![], vec![], vec![], vec![]],
			type_names: vec!["bool".into()],
			type_entries: vec![Some(0)],
			class_icons: vec![Some(7)],
			updates: vec![UpdateData {
				version: "0.1.0.1".into(),
				hash: "abcdefabcdefabcd".into(),
				published: "2026-01-01T00:00:00.000Z".into(),
				date: "Jan 01, 2026".into(),
			}],
			removed: vec![RemovedData {
				kind: EntryKind::Method,
				name: "isA".into(),
				parent: Some("Instance".into()),
				version: "0.1.0.1".into(),
				published: "2026-01-01T00:00:00.000Z".into(),
			}],
		}
	}

	#[test]
	fn round_trips_every_field() {
		let data = sample();
		assert_eq!(IndexData::decode(&data.encode()).unwrap(), data);
	}

	#[test]
	fn rejects_truncated_or_foreign_bytes() {
		let bytes = sample().encode();
		assert!(Index::new(&bytes[..bytes.len() - 3]).is_err());
		assert!(Index::new(b"{\"names\":[]}").is_err());
		let mut corrupted = bytes;
		corrupted[12] = 0xff;
		assert!(Index::new(&corrupted).is_err());
	}
}
