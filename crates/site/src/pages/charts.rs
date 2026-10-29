//! SVG charts for the Statistics page.

use maud::{Markup, html};
use rapi_core::js::{js_number, js_round};

const PLOT_HEIGHT: f64 = 100.0;
const STEPS: [f64; 5] = [1.0, 2.0, 2.5, 5.0, 10.0];

fn num(value: f64) -> String {
	js_number(value)
}

pub fn nice_ceil(value: f64) -> f64 {
	if value <= 0.0 {
		return 1.0;
	}
	let magnitude = 10f64.powf(value.log10().floor());
	let step = STEPS
		.into_iter()
		.find(|step| step * magnitude >= value)
		.unwrap_or(10.0);
	step * magnitude
}

pub fn axis_cap(values: &[f64]) -> f64 {
	let mut sorted = values.to_vec();
	sorted.sort_by(|a, b| b.total_cmp(a));
	let first = sorted.first().copied().unwrap_or(0.0);
	let second = sorted.get(1).copied().unwrap_or(0.0);
	nice_ceil(if first > second * 2.0 && second > 0.0 {
		second
	} else {
		first
	})
}

pub fn percent(value: f64) -> String {
	format!("{}%", num(js_round(value * 1000.0) / 10.0))
}

pub struct ChartTick {
	pub index: usize,
	pub label: String,
}

const MINIMUM_GAP: f64 = 0.06;

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
pub fn year_ticks<'a>(dates: impl ExactSizeIterator<Item = &'a str>) -> Vec<ChartTick> {
	let count = dates.len() as f64;
	let mut ticks: Vec<ChartTick> = Vec::new();
	for (index, date) in dates.enumerate() {
		let year = date.get(..4).unwrap_or(date);
		if ticks.last().is_some_and(|last| last.label == year) {
			continue;
		}
		if ticks
			.last()
			.is_some_and(|last| (index - last.index) as f64 / count < MINIMUM_GAP)
		{
			ticks.pop();
		}
		ticks.push(ChartTick {
			index,
			label: year.to_owned(),
		});
	}
	ticks
}

fn chart_frame(title: &str, summary: &str, legend: Option<Markup>, children: &Markup) -> Markup {
	html! {
		figure class="flex min-w-0 flex-col gap-3" {
			figcaption class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1" {
				span class="font-medium text-fg" { (title) }
				span class="text-fg-muted" { (summary) }
			}
			@if let Some(legend) = legend { (legend) }
			(children)
		}
	}
}

fn chart_row(children: &Markup) -> Markup {
	html! {
		div class="flex gap-2" {
			div class="w-8 shrink-0" {}
			div class="relative h-4 min-w-0 flex-1" { (children) }
		}
	}
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
fn chart_ticks(ticks: &[ChartTick], count: usize) -> Markup {
	html! {
		div aria-hidden="true" class="text-xs text-fg-faint" {
			@for tick in ticks {
				span class="absolute top-0 border-l border-line pl-1" style={ "left:" (percent(tick.index as f64 / count as f64)) } { (tick.label) }
			}
		}
	}
}

pub struct Column {
	pub label: String,
	pub value: f64,
	pub hint: String,
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
pub fn column_chart(title: &str, summary: &str, columns: &[Column]) -> Markup {
	let values: Vec<f64> = columns.iter().map(|column| column.value).collect();
	let cap = axis_cap(&values);
	let scale = PLOT_HEIGHT / cap;
	let count = columns.len();
	let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
	let min = values.iter().copied().fold(f64::INFINITY, f64::min);
	let content = html! {
		div class="flex flex-col" {
			div class="h-4" {}
			div class="flex gap-2" {
				div aria-hidden="true" class="relative h-32 w-8 shrink-0 text-right text-xs text-fg-faint" {
					span class="absolute top-0 right-0 -translate-y-1/2" { (num(cap)) }
					span class="absolute right-0 bottom-0 translate-y-1/2" { "0" }
				}
				div class="relative h-32 min-w-0 flex-1" {
					svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-32" {
						line class="stroke-line-strong" x1="0" x2=(count) y1=(num(PLOT_HEIGHT)) y2=(num(PLOT_HEIGHT)) vector-effect="non-scaling-stroke" {}
						@for (index, column) in columns.iter().enumerate() {
							@let height = (column.value * scale).min(PLOT_HEIGHT);
							g data-tooltip=(column.hint) {
								rect data-hit="" x=(index) y="0" width="1" height=(num(PLOT_HEIGHT)) {}
								rect class="fill-fg-muted" x=(num(index as f64 + 0.25)) y=(num(PLOT_HEIGHT - height)) width="0.5" height=(num(height)) {}
							}
						}
					}
					@for (index, column) in columns.iter().enumerate() {
						@if column.value == max || column.value == min {
							span aria-hidden="true" class="absolute -translate-x-1/2 -translate-y-full pb-1 text-xs text-fg-muted"
								style={ "left:" (percent((index as f64 + 0.5) / count as f64)) ";top:" (percent(1.0 - (column.value * scale).min(PLOT_HEIGHT) / PLOT_HEIGHT)) } {
								(num(column.value))
							}
						}
					}
				}
			}
			div class="h-1" {}
			(chart_row(&html! {
				div aria-hidden="true" class="flex text-xs text-fg-faint" {
					@for column in columns { span class="min-w-0 flex-1 text-center" { (column.label) } }
				}
			}))
		}
	};
	chart_frame(title, summary, None, &content)
}

pub struct BarColumn {
	pub up: f64,
	pub down: f64,
	pub hint: String,
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
fn overflow(value: f64, index: usize, count: usize, up: bool) -> Markup {
	html! {
		span aria-hidden="true" class={ "absolute -translate-x-1/2 font-mono text-xs text-fg-muted " (if up { "bottom-0" } else { "top-0" }) }
			style={ "left:" (percent((index as f64 + 0.5) / count as f64)) } {
			(num(value))
		}
	}
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
pub fn diverging_bar_chart(
	title: &str,
	summary: &str,
	labels: (&str, &str),
	columns: &[BarColumn],
	ticks: &[ChartTick],
) -> Markup {
	let ups: Vec<f64> = columns.iter().map(|column| column.up).collect();
	let downs: Vec<f64> = columns.iter().map(|column| column.down).collect();
	let up_cap = axis_cap(&ups);
	let down_cap = axis_cap(&downs);
	let scale = PLOT_HEIGHT / (up_cap + down_cap);
	let zero = up_cap * scale;
	let count = columns.len();
	let legend = html! {
		ul class="flex flex-wrap gap-x-4 text-fg-muted" {
			li class="flex items-center gap-2" { span aria-hidden="true" class="size-2 rounded-xs bg-success" {} (labels.0) }
			li class="flex items-center gap-2" { span aria-hidden="true" class="size-2 rounded-xs bg-danger" {} (labels.1) }
		}
	};
	let content = html! {
		div class="flex flex-col" {
			(chart_row(&html! {
				@for (index, column) in columns.iter().enumerate() {
					@if column.up * scale > zero { (overflow(column.up, index, count, true)) }
				}
			}))
			div class="flex gap-2" {
				div aria-hidden="true" class="relative h-40 w-8 shrink-0 text-right text-xs text-fg-faint" {
					span class="absolute top-0 right-0 -translate-y-1/2" { (num(up_cap)) }
					span class="absolute right-0 -translate-y-1/2" style={ "top:" (percent(zero / PLOT_HEIGHT)) } { "0" }
					span class="absolute right-0 bottom-0 translate-y-1/2" { (num(down_cap)) }
				}
				svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-40 min-w-0 flex-1" {
					line class="stroke-line-strong" x1="0" x2=(count) y1=(num(zero)) y2=(num(zero)) vector-effect="non-scaling-stroke" {}
					@for (index, column) in columns.iter().enumerate() {
						@let up = (column.up * scale).min(zero);
						@let down = (column.down * scale).min(PLOT_HEIGHT - zero);
						g data-tooltip=(column.hint) {
							rect data-hit="" x=(index) y="0" width="1" height=(num(PLOT_HEIGHT)) {}
							@if up > 0.0 {
								rect class="fill-success" x=(num(index as f64 + 0.2)) y=(num(zero - up)) width="0.6" height=(num(up)) {}
							}
							@if down > 0.0 {
								rect class="fill-danger" x=(num(index as f64 + 0.2)) y=(num(zero)) width="0.6" height=(num(down)) {}
							}
						}
					}
				}
			}
			(chart_row(&html! {
				@for (index, column) in columns.iter().enumerate() {
					@if column.down * scale > PLOT_HEIGHT - zero { (overflow(column.down, index, count, false)) }
				}
			}))
			(chart_row(&chart_ticks(ticks, count)))
		}
	};
	chart_frame(title, summary, Some(legend), &content)
}

pub struct LinePoint {
	pub value: f64,
	pub hint: String,
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
pub fn line_chart(title: &str, summary: &str, points: &[LinePoint], ticks: &[ChartTick]) -> Markup {
	let max = points
		.iter()
		.map(|point| point.value)
		.fold(f64::NEG_INFINITY, f64::max);
	let min = points
		.iter()
		.map(|point| point.value)
		.fold(f64::INFINITY, f64::min);
	let step = nice_ceil((max - min) / 4.0);
	let top = (max / step).ceil() * step;
	let bottom = (min / step).floor() * step;
	let range = if top - bottom == 0.0 {
		1.0
	} else {
		top - bottom
	};
	let y = |value: f64| PLOT_HEIGHT - (value - bottom) / range * PLOT_HEIGHT;
	let count = points.len();
	let path = points
		.iter()
		.enumerate()
		.map(|(index, point)| format!("{},{}", num(index as f64 + 0.5), num(y(point.value))))
		.collect::<Vec<_>>()
		.join(" ");
	let content = html! {
		div class="flex flex-col" {
			div class="flex gap-2" {
				div aria-hidden="true" class="relative h-32 w-8 shrink-0 text-right text-xs text-fg-faint" {
					span class="absolute top-0 right-0 -translate-y-1/2" { (num(top)) }
					span class="absolute right-0 bottom-0 translate-y-1/2" { (num(bottom)) }
				}
				svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-32 min-w-0 flex-1" {
					line class="stroke-line-subtle" x1="0" x2=(count) y1="0" y2="0" vector-effect="non-scaling-stroke" {}
					line class="stroke-line" x1="0" x2=(count) y1=(num(PLOT_HEIGHT)) y2=(num(PLOT_HEIGHT)) vector-effect="non-scaling-stroke" {}
					@for (index, point) in points.iter().enumerate() {
						rect data-hit="" data-tooltip=(point.hint) x=(index) y="0" width="1" height=(num(PLOT_HEIGHT)) {}
					}
					polyline class="pointer-events-none fill-none stroke-fg-muted stroke-2" points=(path) vector-effect="non-scaling-stroke" stroke-linejoin="round" {}
				}
			}
			div class="h-2" {}
			(chart_row(&chart_ticks(ticks, count)))
		}
	};
	chart_frame(title, summary, None, &content)
}
