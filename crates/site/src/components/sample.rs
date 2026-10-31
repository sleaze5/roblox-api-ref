use maud::{Markup, html};
use rapi_core::api::doc_link_hint::{doc_link_hint, doc_link_kind};
use rapi_core::model::{CodeSample, SampleToken};

use crate::components::code::tone_class;
use crate::components::doc::{class_icon_index, doc_link_href};
use crate::context::Context;

fn sample_token(cx: &Context, token: &SampleToken) -> Markup {
	let tone = tone_class(token.tone);
	html! {
		@match &token.link {
			Some(link) => a href=(doc_link_href(link)) class=[tone] data-tooltip=(doc_link_hint(link)) data-tooltip-kind=[doc_link_kind(link)] data-tooltip-icon=[class_icon_index(cx, link)] { (token.text) },
			None => @match tone {
				Some(tone) => span class=(tone) { (token.text) },
				None => (token.text),
			},
		}
	}
}

pub fn code_sample(cx: &Context, sample: Option<&CodeSample>) -> Markup {
	html! {
		@if let Some(sample) = sample {
			figure class="code-sample" {
				figcaption { "Code sample" }
				pre { code { @for token in &sample.tokens { (sample_token(cx, token)) } } }
			}
		}
	}
}
