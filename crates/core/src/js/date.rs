const MONTHS: [&str; 12] = [
	"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
const DAY_MS: i64 = 86_400_000;

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
	let year = if month <= 2 { year - 1 } else { year };
	let era = year.div_euclid(400);
	let year_of_era = year - era * 400;
	let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
	let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
	era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
	let days = days + 719_468;
	let era = days.div_euclid(146_097);
	let day_of_era = days - era * 146_097;
	let year_of_era =
		(day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
	let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
	let month_index = (5 * day_of_year + 2) / 153;
	let day = day_of_year - (153 * month_index + 2) / 5 + 1;
	let month = if month_index < 10 { month_index + 3 } else { month_index - 9 };
	let year = year_of_era + era * 400 + i64::from(month <= 2);
	(year, month, day)
}

fn number(text: &str) -> Option<i64> {
	if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
		return None;
	}
	text.parse().ok()
}

pub fn parse_iso(text: &str) -> Option<i64> {
	let (date, time) = text.split_once('T').unwrap_or((text, ""));
	let mut parts = date.splitn(3, '-');
	let year = number(parts.next()?)?;
	let month = parts.next().map_or(Some(1), number)?;
	let day = parts.next().map_or(Some(1), number)?;
	let time = time.strip_suffix('Z').unwrap_or(time);
	let (clock, fraction) = time.split_once('.').unwrap_or((time, ""));
	let mut clock_parts = clock.split(':');
	let hours = clock_parts.next().filter(|part| !part.is_empty()).map_or(Some(0), number)?;
	let minutes = clock_parts.next().map_or(Some(0), number)?;
	let seconds = clock_parts.next().map_or(Some(0), number)?;
	let millis = if fraction.is_empty() {
		0
	} else {
		number(&format!("{fraction:0<3}")[..3])?
	};
	let days = days_from_civil(year, month, day);
	Some(days * DAY_MS + ((hours * 60 + minutes) * 60 + seconds) * 1000 + millis)
}

fn month_name(month: i64) -> &'static str {
	MONTHS[usize::try_from(month - 1).unwrap_or_default() % 12]
}

fn parts(iso: &str) -> (i64, i64, i64) {
	civil_from_days(parse_iso(iso).unwrap_or_default().div_euclid(DAY_MS))
}

pub fn format_date(iso: &str) -> String {
	let (year, month, day) = parts(iso);
	format!("{} {day:02}, {year}", month_name(month))
}

pub fn format_short_date(iso: &str) -> String {
	let (_, month, day) = parts(iso);
	format!("{} {day:02}", month_name(month))
}

pub fn format_month(iso: &str) -> String {
	let (_, month, _) = parts(iso);
	month_name(month).to_owned()
}

pub fn days_between(from: &str, to: &str) -> f64 {
	let difference = parse_iso(to).unwrap_or_default() - parse_iso(from).unwrap_or_default();
	#[expect(clippy::cast_precision_loss, reason = "timestamps fit in 53 bits")]
	let days = difference as f64 / 86_400_000.0;
	days
}

pub fn iso_string(millis: i64) -> String {
	let (year, month, day) = civil_from_days(millis.div_euclid(DAY_MS));
	let time = millis.rem_euclid(DAY_MS);
	format!(
		"{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
		time / 3_600_000,
		time / 60_000 % 60,
		time / 1000 % 60,
		time % 1000
	)
}

pub fn utc_string(millis: i64) -> String {
	let days = millis.div_euclid(DAY_MS);
	let (year, month, day) = civil_from_days(days);
	let time = millis.rem_euclid(DAY_MS) / 1000;
	let weekday = WEEKDAYS[usize::try_from(days.rem_euclid(7)).unwrap_or_default()];
	format!(
		"{weekday}, {day:02} {} {year} {:02}:{:02}:{:02} GMT",
		month_name(month),
		time / 3600,
		time / 60 % 60,
		time % 60
	)
}

pub fn parse_http_date(text: &str) -> Option<i64> {
	let mut words = text.split_whitespace().skip(1);
	let day = number(words.next()?)?;
	let month_word = words.next()?;
	let month = MONTHS.iter().position(|name| *name == month_word)?;
	let year = number(words.next()?)?;
	let mut clock = words.next()?.split(':');
	let hours = number(clock.next()?)?;
	let minutes = number(clock.next()?)?;
	let seconds = number(clock.next()?)?;
	let month = i64::try_from(month).ok()? + 1;
	Some(days_from_civil(year, month, day) * DAY_MS + ((hours * 60 + minutes) * 60 + seconds) * 1000)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formats_dates_like_intl_in_utc() {
		assert_eq!(format_date("2026-10-05T23:59:59.000Z"), "Oct 05, 2026");
		assert_eq!(format_short_date("2024-02-29T00:00:00.000Z"), "Feb 29");
		assert_eq!(format_month("2000-12-15T00:00:00Z"), "Dec");
	}

	#[test]
	fn round_trips_timestamps() {
		let millis = parse_iso("2026-10-26T22:29:52.123Z").unwrap();
		assert_eq!(iso_string(millis), "2026-10-26T22:29:52.123Z");
		assert_eq!(parse_iso("1970-01-01T00:00:00.000Z"), Some(0));
		assert_eq!(utc_string(millis), "Mon, 26 Oct 2026 22:29:52 GMT");
		assert_eq!(utc_string(0), "Thu, 01 Jan 1970 00:00:00 GMT");
		assert_eq!(
			parse_http_date("Mon, 26 Oct 2026 22:29:52 GMT"),
			Some(millis - 123)
		);
	}

	#[test]
	fn measures_days() {
		assert!(
			(days_between("2026-01-01T00:00:00.000Z", "2026-01-02T12:00:00.000Z") - 1.5).abs()
				< f64::EPSILON
		);
	}
}
