use std::fmt::Write as _;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
	ArrowSquareOut,
	ArrowUUpLeft,
	Books,
	BracketsCurly,
	CaretDown,
	CaretRight,
	ChartBar,
	Check,
	CheckCircle,
	CircleNotch,
	ClockCounterClockwise,
	Copy,
	Cube,
	EnvelopeSimple,
	Folders,
	Function,
	FunnelSimple,
	GitDiff,
	GitFork,
	Hash,
	Info,
	Lightning,
	LinkSimple,
	List,
	ListBullets,
	LockSimple,
	MagnifyingGlass,
	Memory,
	Prohibit,
	PuzzlePiece,
	Shield,
	SignIn,
	SignOut,
	SlidersHorizontal,
	Tag,
	TreeStructure,
	Warning,
	X,
}

impl Icon {
	pub const ALL: [Self; 38] = [
		Self::ArrowSquareOut,
		Self::ArrowUUpLeft,
		Self::Books,
		Self::BracketsCurly,
		Self::CaretDown,
		Self::CaretRight,
		Self::ChartBar,
		Self::Check,
		Self::CheckCircle,
		Self::CircleNotch,
		Self::ClockCounterClockwise,
		Self::Copy,
		Self::Cube,
		Self::EnvelopeSimple,
		Self::Folders,
		Self::Function,
		Self::FunnelSimple,
		Self::GitDiff,
		Self::GitFork,
		Self::Hash,
		Self::Info,
		Self::Lightning,
		Self::LinkSimple,
		Self::List,
		Self::ListBullets,
		Self::LockSimple,
		Self::MagnifyingGlass,
		Self::Memory,
		Self::Prohibit,
		Self::PuzzlePiece,
		Self::Shield,
		Self::SignIn,
		Self::SignOut,
		Self::SlidersHorizontal,
		Self::Tag,
		Self::TreeStructure,
		Self::Warning,
		Self::X,
	];

	pub const MASKS: [Self; 21] = [
		Self::ArrowSquareOut,
		Self::ArrowUUpLeft,
		Self::BracketsCurly,
		Self::CaretDown,
		Self::CaretRight,
		Self::Check,
		Self::ClockCounterClockwise,
		Self::Cube,
		Self::Folders,
		Self::Function,
		Self::GitDiff,
		Self::GitFork,
		Self::Hash,
		Self::Lightning,
		Self::LinkSimple,
		Self::ListBullets,
		Self::Memory,
		Self::PuzzlePiece,
		Self::Shield,
		Self::SlidersHorizontal,
		Self::Tag,
	];

	pub const fn name(self) -> &'static str {
		match self {
			Self::ArrowSquareOut => "arrow-square-out",
			Self::ArrowUUpLeft => "arrow-u-up-left",
			Self::Books => "books",
			Self::BracketsCurly => "brackets-curly",
			Self::CaretDown => "caret-down",
			Self::CaretRight => "caret-right",
			Self::ChartBar => "chart-bar",
			Self::Check => "check",
			Self::CheckCircle => "check-circle",
			Self::CircleNotch => "circle-notch",
			Self::ClockCounterClockwise => "clock-counter-clockwise",
			Self::Copy => "copy",
			Self::Cube => "cube",
			Self::EnvelopeSimple => "envelope-simple",
			Self::Folders => "folders",
			Self::Function => "function",
			Self::FunnelSimple => "funnel-simple",
			Self::GitDiff => "git-diff",
			Self::GitFork => "git-fork",
			Self::Hash => "hash",
			Self::Info => "info",
			Self::Lightning => "lightning",
			Self::LinkSimple => "link-simple",
			Self::List => "list",
			Self::ListBullets => "list-bullets",
			Self::LockSimple => "lock-simple",
			Self::MagnifyingGlass => "magnifying-glass",
			Self::Memory => "memory",
			Self::Prohibit => "prohibit",
			Self::PuzzlePiece => "puzzle-piece",
			Self::Shield => "shield",
			Self::SignIn => "sign-in",
			Self::SignOut => "sign-out",
			Self::SlidersHorizontal => "sliders-horizontal",
			Self::Tag => "tag",
			Self::TreeStructure => "tree-structure",
			Self::Warning => "warning",
			Self::X => "x",
		}
	}
}

pub fn read_icon(phosphor: &Path, icon: Icon) -> std::io::Result<String> {
	let text = std::fs::read_to_string(phosphor.join(format!("{}.svg", icon.name())))?;
	Ok(text.trim().to_owned())
}

fn svg_parts(svg: &str) -> Option<(&str, &str)> {
	let open_end = svg.find('>')?;
	let open = &svg[..open_end];
	let view_box_start = open.find("viewBox=\"")? + "viewBox=\"".len();
	let view_box_end = view_box_start + open[view_box_start..].find('"')?;
	let body = svg[open_end + 1..].strip_suffix("</svg>")?;
	Some((&open[view_box_start..view_box_end], body))
}

pub fn build_icon_sprite(phosphor: &Path) -> std::io::Result<String> {
	let mut sprite = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\">");
	for icon in Icon::ALL {
		let svg = read_icon(phosphor, icon)?;
		let (view_box, body) = svg_parts(&svg).ok_or_else(|| {
			std::io::Error::other(format!("Unexpected SVG format in {}.svg", icon.name()))
		})?;
		let _ = write!(
			sprite,
			"<symbol id=\"{}\" viewBox=\"{view_box}\">{body}</symbol>",
			icon.name()
		);
	}
	sprite.push_str("</svg>\n");
	Ok(sprite)
}

pub fn build_favicon(phosphor: &Path) -> std::io::Result<String> {
	let svg = read_icon(phosphor, Icon::Cube)?;
	let body = svg_parts(&svg).map_or("", |(_, body)| body);
	Ok(format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\"><rect width=\"256\" height=\"256\" rx=\"56\" fill=\"#111111\"/><g fill=\"#ffffff\" transform=\"translate(32 32) scale(.75)\">{body}</g></svg>\n"
	))
}
