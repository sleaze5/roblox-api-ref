pub fn js_number(value: f64) -> String {
	let mut buffer = ryu_js::Buffer::new();
	buffer.format(value).to_owned()
}

pub fn js_round(value: f64) -> f64 {
	(value + 0.5).floor()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formats_numbers_like_javascript() {
		assert_eq!(js_number(1.0), "1");
		assert_eq!(js_number(87.333_333_333_333_33), "87.33333333333333");
		assert_eq!(js_number(3.552_713_678_800_501e-15), "3.552713678800501e-15");
		assert_eq!(js_number(-0.0), "0");
		assert_eq!(js_number(1e21), "1e+21");
		assert_eq!(js_number(0.1 + 0.2), "0.30000000000000004");
	}

	#[test]
	fn rounds_halves_up_like_math_round() {
		assert!((js_round(2.5) - 3.0).abs() < f64::EPSILON);
		assert!((js_round(-2.5) + 2.0).abs() < f64::EPSILON);
		assert!((js_round(-2.6) + 3.0).abs() < f64::EPSILON);
	}
}
