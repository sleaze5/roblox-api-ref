use maud::{Markup, html};
use rapi_core::model::{CodeSample, SampleToken};

use crate::components::code::tone_class;

fn sample_token(token: &SampleToken) -> Markup {
	html! {
		@match tone_class(token.tone) {
			Some(tone) => span class=(tone) { (token.text) },
			None => (token.text),
		}
	}
}

pub fn code_sample(sample: Option<&CodeSample>) -> Markup {
	html! {
		@if let Some(sample) = sample {
			figure data-code-sample="" class="code-sample" {
				figcaption { "Code sample" }
				div class="relative" {
					pre {
						code style={ "--line-digits:" (sample.lines.len().to_string().len()) } {
							@for line in &sample.lines {
								span data-code-line="" class="code-line" {
									span class="code-text" { @for token in line { (sample_token(token)) } }
								}
							}
						}
					}
					div data-code-actions="" hidden class="code-actions" {
						button type="button" data-wrap-code="" aria-pressed="false" aria-label="Wrap lines" data-tooltip="Wrap lines" class="wrap-code" {}
						button type="button" data-copy-code="" aria-label="Copy code" data-tooltip="Copy code" class="copy-code" {}
					}
				}
			}
		}
	}
}
