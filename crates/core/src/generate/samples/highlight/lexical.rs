use full_moon::tokenizer::{Symbol, TokenType};

use crate::api::code::CodeTone;

const fn symbol_tone(symbol: Symbol) -> CodeTone {
	match symbol {
		Symbol::True | Symbol::False | Symbol::Nil => CodeTone::Constant,
		Symbol::Colon
		| Symbol::Comma
		| Symbol::Dot
		| Symbol::LeftBrace
		| Symbol::LeftBracket
		| Symbol::LeftParen
		| Symbol::RightBrace
		| Symbol::RightBracket
		| Symbol::RightParen
		| Symbol::Semicolon
		| Symbol::TwoColons => CodeTone::Plain,
		_ => CodeTone::Keyword,
	}
}

pub fn token_tone(token_type: &TokenType) -> CodeTone {
	match token_type {
		TokenType::Symbol { symbol } => symbol_tone(*symbol),
		TokenType::Number { .. } => CodeTone::Constant,
		TokenType::StringLiteral { .. } | TokenType::InterpolatedString { .. } => CodeTone::String,
		TokenType::SingleLineComment { .. } | TokenType::MultiLineComment { .. } => {
			CodeTone::Comment
		}
		_ => CodeTone::Plain,
	}
}
