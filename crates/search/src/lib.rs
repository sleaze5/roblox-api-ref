mod complete;
mod engine;
mod format;
mod json;
mod lookup;
mod matcher;
mod query;
mod score;
mod top;
mod view;

pub use complete::complete;
pub use engine::{Engine, SearchResult, TypeUsers};
pub use format::{
	EntryKind, IndexData, IndexError, RemovedData, Role, USAGE_LIMIT, UpdateData, decode_check,
	flags,
};
pub use lookup::{match_removed, match_updates};
pub use matcher::Matcher;
pub use query::{ParsedQuery, parse_query};
pub use score::{is_boundary, match_positions, score_name};
pub use view::{
	Group, GroupKey, SearchView, group_results, removed_href, render_view, result_href,
};

pub fn luau_name(name: &str) -> &str {
	match name {
		"int" | "int64" | "float" | "double" => "number",
		"bool" => "boolean",
		"Variant" => "any",
		"ContentId" | "ProtectedString" | "BinaryString" => "string",
		"CoordinateFrame" | "OptionalCoordinateFrame" => "CFrame",
		other => other,
	}
}
