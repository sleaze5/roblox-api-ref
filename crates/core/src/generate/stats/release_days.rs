use crate::js::{days_between, js_round, weekday};
use crate::model::{GAP_DAYS, GapBucket, Update, WeekdayTrend};

pub fn weekday_trends(updates: &[Update]) -> Vec<WeekdayTrend> {
	let mut days: Vec<WeekdayTrend> = (0..7)
		.map(|weekday| WeekdayTrend {
			weekday,
			updates: 0,
			changes: 0,
		})
		.collect();
	for update in updates {
		let day = &mut days[weekday(&update.published)];
		day.updates += 1;
		day.changes += update.counts.total();
	}
	days
}

pub fn gap_buckets(updates: &[Update]) -> Vec<GapBucket> {
	let mut buckets: Vec<GapBucket> = (0..=GAP_DAYS)
		.map(|days| GapBucket { days, updates: 0 })
		.collect();
	let mut dates: Vec<&str> = updates
		.iter()
		.map(|update| update.published.as_str())
		.collect();
	dates.sort_unstable();
	for pair in dates.windows(2) {
		#[expect(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "gaps are a few days"
		)]
		let days = js_round(days_between(pair[0], pair[1])).max(0.0) as u32;
		buckets[days.min(GAP_DAYS) as usize].updates += 1;
	}
	buckets
}
