#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessTier {
	Game,
	Plugin,
	Command,
	Roblox,
}

impl AccessTier {
	pub const ALL: [Self; 4] = [Self::Game, Self::Plugin, Self::Command, Self::Roblox];

	pub const FILTERS: [Self; 3] = [Self::Game, Self::Plugin, Self::Command];

	pub const fn code(self) -> u8 {
		self as u8
	}

	pub const fn name(self) -> &'static str {
		match self {
			Self::Game => "game",
			Self::Plugin => "plugin",
			Self::Command => "command",
			Self::Roblox => "roblox",
		}
	}

	pub fn filter(name: &str) -> Option<Self> {
		Self::FILTERS.into_iter().find(|tier| tier.name() == name)
	}
}
