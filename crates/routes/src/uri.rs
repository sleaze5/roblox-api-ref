const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn is_unreserved(byte: u8) -> bool {
	byte.is_ascii_alphanumeric()
		|| matches!(
			byte,
			b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
		)
}

pub fn encode_uri_component(text: &str) -> String {
	let mut encoded = String::with_capacity(text.len());
	for &byte in text.as_bytes() {
		if is_unreserved(byte) {
			encoded.push(char::from(byte));
		} else {
			encoded.push('%');
			encoded.push(char::from(HEX[usize::from(byte >> 4)]));
			encoded.push(char::from(HEX[usize::from(byte & 15)]));
		}
	}
	encoded
}

fn hex_value(byte: u8) -> Option<u8> {
	match byte {
		b'0'..=b'9' => Some(byte - b'0'),
		b'a'..=b'f' => Some(byte - b'a' + 10),
		b'A'..=b'F' => Some(byte - b'A' + 10),
		_ => None,
	}
}

pub fn decode_uri_component(text: &str) -> Option<String> {
	let bytes = text.as_bytes();
	let mut decoded = Vec::with_capacity(bytes.len());
	let mut index = 0;
	while index < bytes.len() {
		if bytes[index] == b'%' {
			let high = hex_value(*bytes.get(index + 1)?)?;
			let low = hex_value(*bytes.get(index + 2)?)?;
			decoded.push(high << 4 | low);
			index += 3;
		} else {
			decoded.push(bytes[index]);
			index += 1;
		}
	}
	String::from_utf8(decoded).ok()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encodes_like_javascript() {
		assert_eq!(encode_uri_component("Body Parts"), "Body%20Parts");
		assert_eq!(encode_uri_component("a/b?c#d&e"), "a%2Fb%3Fc%23d%26e");
		assert_eq!(encode_uri_component("-_.!~*'()"), "-_.!~*'()");
		assert_eq!(encode_uri_component("é"), "%C3%A9");
		assert_eq!(encode_uri_component("0.745.0.7451137"), "0.745.0.7451137");
	}

	#[test]
	fn decodes_what_it_encodes_and_rejects_malformed_escapes() {
		for text in ["Physics Response", "é/ü", "100%"] {
			assert_eq!(
				decode_uri_component(&encode_uri_component(text)).as_deref(),
				Some(text)
			);
		}
		assert_eq!(decode_uri_component("%E0%A4%A"), None);
		assert_eq!(decode_uri_component("%FF"), None);
		assert_eq!(decode_uri_component("%zz"), None);
	}
}
