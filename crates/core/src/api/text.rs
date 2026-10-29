use super::release::release_name;
use crate::js::format_month;
use crate::model::Update;

pub const UPDATES_LEAD: &str = "Every change to the Roblox Engine API between the Roblox Studio versions that this site has published, newest first.";

pub const REMOVED_LEAD: &str = "Classes and enums that Roblox removed from the API since the oldest version that this site tracks, newest first.";

pub fn update_summary(update: &Update) -> String {
	format!(
		"Changes to the Roblox Engine API from {} to {}.",
		release_name(&update.previous.version),
		release_name(&update.version)
	)
}

pub fn format_month_period(period: &str) -> String {
	format_month(&format!("2000-{period}-15T00:00:00Z"))
}
