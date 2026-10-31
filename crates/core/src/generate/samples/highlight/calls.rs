use full_moon::ast::{Expression, FunctionArgs};
use full_moon::tokenizer::{TokenReference, TokenType};

pub const CLASS_NAME_METHODS: [&str; 8] = [
	"FindFirstAncestorOfClass",
	"FindFirstAncestorWhichIsA",
	"FindFirstChildOfClass",
	"FindFirstChildWhichIsA",
	"FindService",
	"GetService",
	"IsA",
	"IsDescendantOfClass",
];

pub const RETURNS_NAMED_CLASS: [&str; 6] = [
	"FindFirstAncestorOfClass",
	"FindFirstAncestorWhichIsA",
	"FindFirstChildOfClass",
	"FindFirstChildWhichIsA",
	"FindService",
	"GetService",
];

pub const CONNECTIONS: [&str; 3] = ["Connect", "ConnectParallel", "Once"];

pub fn arguments(args: &FunctionArgs) -> Vec<&Expression> {
	match args {
		FunctionArgs::Parentheses { arguments, .. } => arguments.iter().collect(),
		_ => Vec::new(),
	}
}

pub fn string_argument(args: &FunctionArgs) -> Option<(&TokenReference, String)> {
	let token = match args {
		FunctionArgs::String(token) => token,
		FunctionArgs::Parentheses { .. } => match arguments(args).first() {
			Some(Expression::String(token)) => token,
			_ => return None,
		},
		_ => return None,
	};
	match token.token().token_type() {
		TokenType::StringLiteral { literal, .. } => Some((token, literal.to_string())),
		_ => None,
	}
}
