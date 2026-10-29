use super::members::access_rank;
use crate::model::{AccessTier, KindCounts, Member, Security};

pub const fn access_tier_label(tier: AccessTier) -> &'static str {
	match tier {
		AccessTier::Game => "Game scripts",
		AccessTier::Plugin => "Plugins only",
		AccessTier::Command => "Command bar only",
		AccessTier::Roblox => "Roblox only",
	}
}

pub const fn access_tier_hint(tier: AccessTier) -> &'static str {
	match tier {
		AccessTier::Game => {
			"Members that game scripts can use, and so can plugins and the command bar."
		}
		AccessTier::Plugin => {
			"Members that plugins and the command bar can use, but game scripts cannot."
		}
		AccessTier::Command => {
			"Members that the command bar can use, but plugins and game scripts cannot."
		}
		AccessTier::Roblox => "Members that only Roblox's own scripts and the engine can use.",
	}
}

pub const fn access_tier(member: &Member) -> AccessTier {
	match access_rank(member) {
		0 => AccessTier::Game,
		1 => AccessTier::Plugin,
		2 => AccessTier::Command,
		_ => AccessTier::Roblox,
	}
}

pub fn tier_counts(members: &[Member]) -> [u32; 4] {
	let mut counts = [0; 4];
	for member in members {
		counts[access_tier(member) as usize] += 1;
	}
	counts
}

pub fn tier_rows(members: &[&Member]) -> Vec<(AccessTier, KindCounts)> {
	let mut rows: Vec<(AccessTier, KindCounts)> = AccessTier::ALL
		.into_iter()
		.map(|tier| (tier, KindCounts::default()))
		.collect();
	for member in members {
		rows[access_tier(member) as usize].1.bump(member.kind());
	}
	rows
}

pub const SECURITY_ACCESS: [(Security, &[u32], &[Security]); 5] = [
	(
		Security::RobloxSecurity,
		&[7, 8],
		&[
			Security::None,
			Security::PluginSecurity,
			Security::LocalUserSecurity,
			Security::WritePlayerSecurity,
			Security::RobloxScriptSecurity,
			Security::RobloxSecurity,
			Security::NotAccessibleSecurity,
		],
	),
	(
		Security::RobloxScriptSecurity,
		&[3, 6],
		&[
			Security::None,
			Security::PluginSecurity,
			Security::LocalUserSecurity,
			Security::RobloxScriptSecurity,
		],
	),
	(
		Security::LocalUserSecurity,
		&[1, 4],
		&[
			Security::None,
			Security::PluginSecurity,
			Security::LocalUserSecurity,
		],
	),
	(
		Security::PluginSecurity,
		&[5],
		&[Security::None, Security::PluginSecurity],
	),
	(Security::None, &[2], &[Security::None]),
];

pub fn identities_allowing(security: Security) -> Vec<u32> {
	let mut identities: Vec<u32> = SECURITY_ACCESS
		.iter()
		.filter(|(_, _, allowed)| allowed.contains(&security))
		.flat_map(|(_, identities, _)| identities.iter().copied())
		.collect();
	identities.sort_unstable();
	identities
}
