use maud::{Markup, html};
use rapi_core::js::{js_number, js_round};

const PLOT_HEIGHT: f64 = 100.0;
const GRID_STEPS: [f64; 4] = [1.0, 2.0, 2.5, 5.0];
const GRID_INTERVALS: f64 = 5.0;
const FIT_WIDTHS: [f64; 5] = [328.0, 368.0, 440.0, 560.0, 680.0];
const FIT_LEVELS: [&str; 5] = ["0", "1", "2", "3", "4"];
const AXIS_WIDTH: f64 = 48.0;
const GUTTER_WIDTH: f64 = 40.0;
const CHARACTER_WIDTH: f64 = 7.5;
const TICK_PADDING: f64 = 5.0;
const LABEL_GAP: f64 = 6.0;

fn num(value: f64) -> String {
	js_number(value)
}

fn axis_number(value: f64) -> String {
	if value < 0.0 {
		format!("\u{2212}{}", num(-value))
	} else {
		num(value)
	}
}

pub fn axis_limit(values: &[f64]) -> f64 {
	let mut sorted = values.to_vec();
	sorted.sort_by(|a, b| b.total_cmp(a));
	let first = sorted.first().copied().unwrap_or(0.0);
	let second = sorted.get(1).copied().unwrap_or(0.0);
	if first > second * 2.0 && second > 0.0 {
		second
	} else {
		first
	}
}

pub fn percent(value: f64) -> String {
	format!("{}%", num(js_round(value * 1000.0) / 10.0))
}

fn grid_step(intervals: impl Fn(f64) -> f64) -> f64 {
	(0..16)
		.flat_map(|power| {
			GRID_STEPS
				.into_iter()
				.filter(move |base| power > 0 || base.fract() < f64::EPSILON)
				.map(move |base| base * 10f64.powi(power))
		})
		.find(|step| intervals(*step) <= GRID_INTERVALS)
		.unwrap_or(1.0)
}

fn steps_to(value: f64, step: f64) -> f64 {
	(value / step).ceil().max(0.0)
}

pub struct Grid {
	pub top: f64,
	pub bottom: f64,
	pub step: f64,
}

impl Grid {
	pub fn from_zero(limit: f64) -> Self {
		let step = grid_step(|step| steps_to(limit, step));
		Self {
			top: steps_to(limit, step).max(1.0) * step,
			bottom: 0.0,
			step,
		}
	}

	pub fn around_zero(up: f64, down: f64) -> Self {
		let step = grid_step(|step| steps_to(up, step) + steps_to(down, step));
		Self {
			top: steps_to(up, step).max(1.0) * step,
			bottom: -steps_to(down, step) * step,
			step,
		}
	}

	pub fn spanning(min: f64, max: f64) -> Self {
		let floor = |step: f64| (min / step).floor() * step;
		let step = grid_step(|step| steps_to(max - floor(step), step).max(1.0));
		let bottom = floor(step);
		Self {
			top: bottom + steps_to(max - bottom, step).max(1.0) * step,
			bottom,
			step,
		}
	}

	fn y(&self, value: f64) -> f64 {
		(self.top - value.clamp(self.bottom, self.top)) / (self.top - self.bottom) * PLOT_HEIGHT
	}

	#[expect(
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
		clippy::cast_precision_loss,
		reason = "grids have a handful of lines"
	)]
	fn values(&self) -> Vec<f64> {
		let count = js_round((self.top - self.bottom) / self.step) as usize;
		(0..=count)
			.map(|index| self.step.mul_add(-(index as f64), self.top))
			.collect()
	}
}

pub struct ChartTick {
	pub index: usize,
	pub label: String,
}

pub fn year_ticks<'a>(dates: impl ExactSizeIterator<Item = &'a str>) -> Vec<ChartTick> {
	let mut ticks: Vec<ChartTick> = Vec::new();
	for (index, date) in dates.enumerate() {
		let year = date.get(..4).unwrap_or(date);
		if ticks.last().is_none_or(|last| last.label != year) {
			ticks.push(ChartTick {
				index,
				label: year.to_owned(),
			});
		}
	}
	ticks
}

#[derive(Clone, Copy)]
enum Anchor {
	Start,
	Center,
}

struct Label {
	at: f64,
	width: f64,
}

#[expect(clippy::cast_precision_loss, reason = "labels are short")]
fn text_width(text: &str) -> f64 {
	text.chars().count() as f64 * CHARACTER_WIDTH
}

fn label_span(label: &Label, anchor: Anchor, plot: f64) -> (f64, f64) {
	let start = match anchor {
		Anchor::Start => label.at * plot,
		Anchor::Center => label.width.mul_add(-0.5, label.at * plot),
	};
	(start, start + label.width)
}

fn fit_levels(
	labels: &[Label],
	anchor: Anchor,
	gutter: f64,
	shown: &[bool],
) -> Vec<Option<&'static str>> {
	let mut levels: Vec<Option<usize>> = shown.iter().map(|shown| shown.then_some(0)).collect();
	for (level, figure) in FIT_WIDTHS.into_iter().enumerate() {
		let plot = figure - AXIS_WIDTH - gutter;
		for index in (0..labels.len()).rev() {
			if levels[index].is_some() {
				continue;
			}
			let (start, end) = label_span(&labels[index], anchor, plot);
			let inside = start >= 0.0 && end <= plot + gutter;
			let clear = labels.iter().zip(&levels).all(|(other, placed)| {
				placed.is_none() || {
					let (other_start, other_end) = label_span(other, anchor, plot);
					end + LABEL_GAP <= other_start || other_end + LABEL_GAP <= start
				}
			});
			if inside && clear {
				levels[index] = Some(level);
			}
		}
	}
	levels
		.into_iter()
		.map(|level| level.map(|level| FIT_LEVELS[level]))
		.collect()
}

fn fit_attribute(level: Option<&'static str>) -> Option<&'static str> {
	level.filter(|level| *level != FIT_LEVELS[0])
}

fn chart_frame(title: &str, summary: &str, legend: Option<Markup>, children: &Markup) -> Markup {
	html! {
		figure class="chart flex min-w-0 flex-col gap-3" {
			figcaption class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1" {
				span class="font-medium text-fg" { (title) }
				span class="text-fg-muted" { (summary) }
			}
			@if let Some(legend) = legend { (legend) }
			(children)
		}
	}
}

fn chart_row(gutter: bool, height: &str, children: &Markup) -> Markup {
	html! {
		div class="flex gap-2" {
			div class="w-10 shrink-0" {}
			div class={ "relative min-w-0 flex-1 " (height) } { (children) }
			@if gutter { div class="w-8 shrink-0" {} }
		}
	}
}

fn y_axis(grid: &Grid, height: &str) -> Markup {
	html! {
		div aria-hidden="true" class={ "relative w-10 shrink-0 text-right text-xs text-fg-faint tabular-nums " (height) } {
			@for value in grid.values() {
				span class="absolute right-0 -translate-y-1/2" style={ "top:" (percent(grid.y(value) / PLOT_HEIGHT)) } { (axis_number(value)) }
			}
		}
	}
}

fn grid_lines(grid: &Grid, count: usize) -> Markup {
	html! {
		@for value in grid.values() {
			@let class = if value.abs() < f64::EPSILON { "stroke-line-strong" } else if (value - grid.bottom).abs() < f64::EPSILON { "stroke-line" } else { "stroke-line-subtle" };
			line class=(class) x1="0" x2=(count) y1=(num(grid.y(value))) y2=(num(grid.y(value))) vector-effect="non-scaling-stroke" {}
		}
	}
}

fn year_lines(ticks: &[ChartTick]) -> Markup {
	html! {
		@for tick in ticks.iter().filter(|tick| tick.index > 0) {
			line class="stroke-line-subtle" x1=(tick.index) x2=(tick.index) y1="0" y2=(num(PLOT_HEIGHT)) vector-effect="non-scaling-stroke" {}
		}
	}
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
fn year_labels(ticks: &[ChartTick], count: usize) -> Markup {
	let labels: Vec<Label> = ticks
		.iter()
		.map(|tick| Label {
			at: tick.index as f64 / count as f64,
			width: text_width(&tick.label) + TICK_PADDING,
		})
		.collect();
	let levels = fit_levels(
		&labels,
		Anchor::Start,
		GUTTER_WIDTH,
		&vec![false; ticks.len()],
	);
	html! {
		div aria-hidden="true" class="text-xs text-fg-faint tabular-nums" {
			@for (tick, level) in ticks.iter().zip(levels) {
				@if level.is_some() {
					span data-fit=[fit_attribute(level)] class="absolute top-0 border-l border-line pl-1" style={ "left:" (percent(tick.index as f64 / count as f64)) } { (tick.label) }
				}
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
fn column_labels(texts: &[String], shown: &[bool]) -> Vec<Option<&'static str>> {
	let count = texts.len() as f64;
	let labels: Vec<Label> = texts
		.iter()
		.enumerate()
		.map(|(index, text)| Label {
			at: (index as f64 + 0.5) / count,
			width: text_width(text),
		})
		.collect();
	fit_levels(&labels, Anchor::Center, 0.0, shown)
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
fn value_place(index: usize, count: usize, text: &str) -> (String, &'static str) {
	let plot = FIT_WIDTHS[0] - AXIS_WIDTH;
	let column = plot / count as f64;
	let center = (index as f64 + 0.5) * column;
	let half = text_width(text) / 2.0;
	if center + half > plot {
		(
			percent((index + 1) as f64 / count as f64),
			"pointer-events-none absolute -translate-x-full -translate-y-full pb-1 text-xs text-fg-muted tabular-nums",
		)
	} else if center < half {
		(
			percent(index as f64 / count as f64),
			"pointer-events-none absolute -translate-y-full pb-1 text-xs text-fg-muted tabular-nums",
		)
	} else {
		(
			percent((index as f64 + 0.5) / count as f64),
			"pointer-events-none absolute -translate-x-1/2 -translate-y-full pb-1 text-xs text-fg-muted tabular-nums",
		)
	}
}

#[expect(clippy::cast_precision_loss, reason = "chart indexes are small")]
pub fn column_chart(title: &str, summary: &str, columns: &[Column]) -> Markup {
	let values: Vec<f64> = columns.iter().map(|column| column.value).collect();
	let grid = Grid::from_zero(axis_limit(&values));
	let count = columns.len();
	let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
	let min = values.iter().copied().fold(f64::INFINITY, f64::min);
	let value_texts: Vec<String> = values.iter().map(|value| num(*value)).collect();
	let extremes: Vec<bool> = values
		.iter()
		.map(|value| {
			*value > 0.0 && (value.total_cmp(&max).is_eq() || value.total_cmp(&min).is_eq())
		})
		.collect();
	let value_levels = column_labels(&value_texts, &extremes);
	let name_texts: Vec<String> = columns.iter().map(|column| column.label.clone()).collect();
	let name_levels = column_labels(&name_texts, &vec![false; count]);
	let content = html! {
		div class="flex flex-col" {
			div class="h-4" {}
			div class="flex gap-2" {
				(y_axis(&grid, "h-32"))
				div class="relative h-32 min-w-0 flex-1" {
					svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-32" {
						(grid_lines(&grid, count))
						@for (index, column) in columns.iter().enumerate() {
							@let top = grid.y(column.value);
							g data-tooltip=(column.hint) {
								rect data-hit="" x=(index) y="0" width="1" height=(num(PLOT_HEIGHT)) {}
								rect class="fill-fg-muted" x=(num(index as f64 + 0.25)) y=(num(top)) width="0.5" height=(num(PLOT_HEIGHT - top)) {}
							}
						}
					}
					@for (index, ((column, text), level)) in columns.iter().zip(&value_texts).zip(value_levels).enumerate() {
						@if level.is_some() {
							@let (left, place) = value_place(index, count, text);
							span aria-hidden="true" data-fit=[fit_attribute(level)] class=(place)
								style={ "left:" (left) ";top:" (percent(grid.y(column.value) / PLOT_HEIGHT)) } {
								(text)
							}
						}
					}
				}
			}
			div class="h-1" {}
			(chart_row(false, "h-4", &html! {
				div aria-hidden="true" class="text-xs text-fg-faint" {
					@for (index, (column, level)) in columns.iter().zip(name_levels).enumerate() {
						@if level.is_some() {
							span data-fit=[fit_attribute(level)] class="absolute top-0 -translate-x-1/2 whitespace-nowrap" style={ "left:" (percent((index as f64 + 0.5) / count as f64)) } { (column.label) }
						}
					}
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
		span aria-hidden="true" class={ "absolute -translate-x-1/2 text-xs text-fg-muted tabular-nums " (if up { "bottom-0" } else { "top-0" }) }
			style={ "left:" (percent((index as f64 + 0.5) / count as f64)) } {
			(num(value))
		}
	}
}

fn average_line(grid: &Grid, count: usize, average: f64) -> Markup {
	html! {
		line class="stroke-fg-muted" stroke-dasharray="4 3" x1="0" x2=(count) y1=(num(grid.y(average))) y2=(num(grid.y(average))) vector-effect="non-scaling-stroke" {}
	}
}

fn average_label(grid: &Grid, average: f64, up: bool) -> Markup {
	let place = if up {
		"pointer-events-none absolute left-1 -translate-y-full text-xs text-fg-muted tabular-nums"
	} else {
		"pointer-events-none absolute left-1 text-xs text-fg-muted tabular-nums"
	};
	html! {
		span aria-hidden="true" class=(place) style={ "top:" (percent(grid.y(average) / PLOT_HEIGHT)) } {
			"Average " (num(average.abs()))
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
	let grid = Grid::around_zero(axis_limit(&ups), axis_limit(&downs));
	let zero = grid.y(0.0);
	let count = columns.len();
	let average =
		|values: &[f64]| js_round(values.iter().sum::<f64>() / count.max(1) as f64 * 10.0) / 10.0;
	let averages = (average(&ups), -average(&downs));
	let legend = html! {
		ul class="flex flex-wrap gap-x-4 text-fg-muted" {
			li class="flex items-center gap-2" { span aria-hidden="true" class="size-2 rounded-xs bg-success" {} (labels.0) }
			li class="flex items-center gap-2" { span aria-hidden="true" class="size-2 rounded-xs bg-danger" {} (labels.1) }
		}
	};
	let content = html! {
		div class="flex flex-col" {
			(chart_row(true, "h-4", &html! {
				@for (index, column) in columns.iter().enumerate() {
					@if column.up > grid.top { (overflow(column.up, index, count, true)) }
				}
			}))
			div class="flex gap-2" {
				(y_axis(&grid, "h-40"))
				div class="relative h-40 min-w-0 flex-1" {
					svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-40" {
						(year_lines(ticks))
						(grid_lines(&grid, count))
						@for (index, column) in columns.iter().enumerate() {
							@let up = zero - grid.y(column.up);
							@let down = grid.y(-column.down) - zero;
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
						@if averages.0 > 0.0 { (average_line(&grid, count, averages.0)) }
						@if averages.1 < 0.0 { (average_line(&grid, count, averages.1)) }
					}
					@if averages.0 > 0.0 { (average_label(&grid, averages.0, true)) }
					@if averages.1 < 0.0 { (average_label(&grid, averages.1, false)) }
				}
				div class="w-8 shrink-0" {}
			}
			(chart_row(true, "h-4", &html! {
				@for (index, column) in columns.iter().enumerate() {
					@if -column.down < grid.bottom { (overflow(column.down, index, count, false)) }
				}
			}))
			(chart_row(true, "h-4", &year_labels(ticks, count)))
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
	let grid = Grid::spanning(min, max);
	let count = points.len();
	let path = points
		.iter()
		.enumerate()
		.map(|(index, point)| format!("{},{}", num(index as f64 + 0.5), num(grid.y(point.value))))
		.collect::<Vec<_>>()
		.join(" ");
	let content = html! {
		div class="flex flex-col" {
			div class="flex gap-2" {
				(y_axis(&grid, "h-32"))
				div class="relative h-32 min-w-0 flex-1" {
					svg role="img" aria-label={ (title) ". " (summary) } viewBox={ "0 0 " (count) " " (num(PLOT_HEIGHT)) } preserveAspectRatio="none" class="chart-plot h-32" {
						(year_lines(ticks))
						(grid_lines(&grid, count))
						@for (index, point) in points.iter().enumerate() {
							rect data-hit="" data-tooltip=(point.hint) x=(index) y="0" width="1" height=(num(PLOT_HEIGHT)) {}
						}
						polyline class="pointer-events-none fill-none stroke-fg-muted stroke-2" points=(path) vector-effect="non-scaling-stroke" stroke-linejoin="round" {}
					}
					@if let Some(last) = points.last() {
						span aria-hidden="true" class="absolute left-full -translate-y-1/2 pl-2 text-xs font-medium text-fg tabular-nums" style={ "top:" (percent(grid.y(last.value) / PLOT_HEIGHT)) } {
							(num(last.value))
						}
					}
				}
				div class="w-8 shrink-0" {}
			}
			div class="h-2" {}
			(chart_row(true, "h-4", &year_labels(ticks, count)))
		}
	};
	chart_frame(title, summary, None, &content)
}

pub struct Share {
	pub label: String,
	pub part: f64,
	pub total: f64,
}

pub fn share_chart(title: &str, summary: &str, rows: &[Share]) -> Markup {
	let content = html! {
		ul role="img" aria-label={ (title) ". " (summary) } class="flex flex-col gap-2" {
			@for row in rows {
				@let ratio = if row.total > 0.0 { row.part / row.total } else { 0.0 };
				li data-tooltip={ (row.label) ": " (num(row.part)) " of " (num(row.total)) } class="grid grid-cols-[8rem_1fr_3.5rem] items-center gap-3" {
					span class="truncate text-fg-muted" { (row.label) }
					span aria-hidden="true" class="relative h-1.5 overflow-hidden rounded-xs bg-line" {
						span class="absolute inset-y-0 left-0 bg-fg-muted" style={ "width:" (percent(ratio)) } {}
					}
					span class="text-right font-mono text-xs text-fg-muted tabular-nums" { (num(js_round(ratio * 100.0))) "%" }
				}
			}
		}
	};
	chart_frame(title, summary, None, &content)
}
