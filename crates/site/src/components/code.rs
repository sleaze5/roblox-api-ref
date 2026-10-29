use maud::{Markup, html};
use rapi_core::api::code::{CodeToken, CodeTone};

const fn tone_class(tone: CodeTone) -> Option<&'static str> {
	match tone {
		CodeTone::SelfName => Some("text-fg-muted"),
		CodeTone::Type => Some("text-code-type"),
		CodeTone::Constant => Some("text-code-constant"),
		CodeTone::String => Some("text-code-string"),
		CodeTone::Function => Some("text-code-function"),
		CodeTone::Name => Some("font-medium text-fg-bright"),
		CodeTone::Parameter | CodeTone::Punctuation | CodeTone::Plain => None,
	}
}

struct Run<'a> {
	href: Option<&'a str>,
	title: Option<&'a str>,
	tokens: Vec<CodeToken>,
}

fn merge_tones(tokens: Vec<CodeToken>) -> Vec<CodeToken> {
	let mut merged: Vec<CodeToken> = Vec::with_capacity(tokens.len());
	for part in tokens {
		match merged.last_mut() {
			Some(last)
				if tone_class(last.tone) == tone_class(part.tone) && part.swatch.is_none() =>
			{
				last.text.push_str(&part.text);
			}
			_ => merged.push(part),
		}
	}
	merged
}

fn runs_of(tokens: &[CodeToken]) -> Vec<Run<'_>> {
	let mut runs: Vec<Run> = Vec::new();
	for part in tokens {
		match runs.last_mut() {
			Some(last) if last.href == part.href.as_deref() => last.tokens.push(part.clone()),
			_ => runs.push(Run {
				href: part.href.as_deref(),
				title: part.title.as_deref(),
				tokens: vec![part.clone()],
			}),
		}
	}
	for run in &mut runs {
		run.tokens = merge_tones(std::mem::take(&mut run.tokens));
	}
	runs
}

fn swatch(color: &str) -> Markup {
	html! {
		span class="mr-1 inline-block size-3 rounded-xs border border-line-strong align-middle" style={ "background-color:" (color) } {}
	}
}

fn plain_tokens(tokens: &[CodeToken]) -> Markup {
	html! {
		@for part in tokens {
			@match (tone_class(part.tone), &part.swatch) {
				(None, None) => (part.text),
				(tone, color) => span class=[tone] {
					@if let Some(color) = color { (swatch(color)) }
					(part.text)
				},
			}
		}
	}
}

pub fn code_tokens(tokens: &[CodeToken]) -> Markup {
	html! {
		@for run in runs_of(tokens) {
			@match run.href {
				None => (plain_tokens(&run.tokens)),
				Some(href) => {
					@if let [only] = run.tokens.as_slice() && only.swatch.is_none() {
						a href=(href) data-tooltip=[run.title] class=[tone_class(only.tone)] { (only.text) }
					} @else {
						a href=(href) data-tooltip=[run.title] { (plain_tokens(&run.tokens)) }
					}
				}
			}
		}
	}
}
