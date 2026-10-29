//! Reads selected entries of a remote zip archive with HTTP range requests,
//! so that a 100 MB archive costs only the bytes of the files it needs.

use std::io::Read;

pub type Result<T> = std::result::Result<T, String>;

/// Reads byte ranges of a file that is too big to download whole.
pub trait RangeReader: Sync {
	fn size(&self) -> u64;
	/// Reads `start..end`.
	fn read(&self, start: u64, end: u64) -> impl Future<Output = Result<Vec<u8>>> + Send;
}

pub struct MemoryReader(pub Vec<u8>);

impl RangeReader for MemoryReader {
	fn size(&self) -> u64 {
		self.0.len() as u64
	}

	async fn read(&self, start: u64, end: u64) -> Result<Vec<u8>> {
		usize::try_from(start)
			.ok()
			.zip(usize::try_from(end).ok())
			.and_then(|(start, end)| self.0.get(start..end))
			.map(<[u8]>::to_vec)
			.ok_or_else(|| format!("Bytes {start}..{end} are out of range."))
	}
}

/// A range that was read once and serves the reads inside it.
struct Window<'a, R: RangeReader> {
	reader: &'a R,
	start: u64,
	bytes: Vec<u8>,
}

impl<R: RangeReader> RangeReader for Window<'_, R> {
	fn size(&self) -> u64 {
		self.reader.size()
	}

	async fn read(&self, start: u64, end: u64) -> Result<Vec<u8>> {
		let window_end = self.start + self.bytes.len() as u64;
		if start >= self.start && end <= window_end {
			let from = usize::try_from(start - self.start).map_err(|error| error.to_string())?;
			let to = usize::try_from(end - self.start).map_err(|error| error.to_string())?;
			return Ok(self.bytes[from..to].to_vec());
		}
		self.reader.read(start, end).await
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZipEntry {
	pub name: String,
	pub method: u16,
	pub encrypted: bool,
	pub crc32: u32,
	pub compressed_size: u64,
	pub size: u64,
	pub local_header_offset: u64,
}

const END_SIGNATURE: u32 = 0x0605_4b50;
const ENTRY_SIGNATURE: u32 = 0x0201_4b50;
const LOCAL_SIGNATURE: u32 = 0x0403_4b50;
const END_RECORD_SIZE: usize = 22;
const MAX_COMMENT_SIZE: u64 = 0xffff;
const ENTRY_HEADER_SIZE: usize = 46;
const LOCAL_HEADER_SIZE: u64 = 30;
const ZIP64_MARKER: u32 = 0xffff_ffff;
const MAX_GAP: u64 = 64 * 1024;

fn u16_at(bytes: &[u8], at: usize) -> u16 {
	u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
	u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// Every entry in the archive's central directory.
pub async fn read_entries(reader: &impl RangeReader) -> Result<Vec<ZipEntry>> {
	let size = reader.size();
	let tail_size = size.min(END_RECORD_SIZE as u64 + MAX_COMMENT_SIZE);
	let tail = reader.read(size - tail_size, size).await?;
	let end = (0..=tail.len().saturating_sub(END_RECORD_SIZE))
		.rev()
		.find(|at| u32_at(&tail, *at) == END_SIGNATURE)
		.ok_or("The zip end of central directory record is missing.")?;
	let directory_size = u32_at(&tail, end + 12);
	let directory_offset = u32_at(&tail, end + 16);
	if directory_size == ZIP64_MARKER || directory_offset == ZIP64_MARKER {
		return Err("ZIP64 archives are not supported.".to_owned());
	}
	let start = u64::from(directory_offset);
	let bytes = reader
		.read(start, start + u64::from(directory_size))
		.await?;
	let mut entries = Vec::new();
	let mut at = 0;
	while at + ENTRY_HEADER_SIZE <= bytes.len() {
		if u32_at(&bytes, at) != ENTRY_SIGNATURE {
			return Err(format!(
				"Unexpected zip central directory record at byte {at}."
			));
		}
		let name_length = usize::from(u16_at(&bytes, at + 28));
		let name_start = at + ENTRY_HEADER_SIZE;
		let name = bytes
			.get(name_start..name_start + name_length)
			.ok_or("A zip entry name runs past the central directory.")?;
		entries.push(ZipEntry {
			name: String::from_utf8_lossy(name).into_owned(),
			method: u16_at(&bytes, at + 10),
			encrypted: u16_at(&bytes, at + 8) & 1 == 1,
			crc32: u32_at(&bytes, at + 16),
			compressed_size: u64::from(u32_at(&bytes, at + 20)),
			size: u64::from(u32_at(&bytes, at + 24)),
			local_header_offset: u64::from(u32_at(&bytes, at + 42)),
		});
		at = name_start
			+ name_length
			+ usize::from(u16_at(&bytes, at + 30))
			+ usize::from(u16_at(&bytes, at + 32));
	}
	Ok(entries)
}

/// The uncompressed bytes of an entry, checked against its size and CRC.
pub async fn read_entry_data(reader: &impl RangeReader, entry: &ZipEntry) -> Result<Vec<u8>> {
	if entry.encrypted {
		return Err(format!("{} is encrypted.", entry.name));
	}
	let header_end = entry.local_header_offset + LOCAL_HEADER_SIZE;
	let header = reader.read(entry.local_header_offset, header_end).await?;
	if u32_at(&header, 0) != LOCAL_SIGNATURE {
		return Err(format!(
			"The zip local header of {} is missing.",
			entry.name
		));
	}
	let data_start = header_end + u64::from(u16_at(&header, 26)) + u64::from(u16_at(&header, 28));
	let compressed = reader
		.read(data_start, data_start + entry.compressed_size)
		.await?;
	let bytes = match entry.method {
		0 => compressed,
		8 => {
			let mut bytes = Vec::with_capacity(usize::try_from(entry.size).unwrap_or_default());
			flate2::read::DeflateDecoder::new(compressed.as_slice())
				.read_to_end(&mut bytes)
				.map_err(|error| format!("{} failed to inflate: {error}", entry.name))?;
			bytes
		}
		method => {
			return Err(format!(
				"{} uses unsupported zip compression method {method}.",
				entry.name
			));
		}
	};
	if bytes.len() as u64 != entry.size || crc32fast::hash(&bytes) != entry.crc32 {
		return Err(format!("{} failed its zip integrity check.", entry.name));
	}
	Ok(bytes)
}

/// The bytes of the entry called `name`.
pub async fn extract_entry(reader: &impl RangeReader, name: &str) -> Result<Vec<u8>> {
	let entries = read_entries(reader).await?;
	let entry = entries
		.iter()
		.find(|entry| entry.name == name)
		.ok_or_else(|| format!("The zip archive does not contain {name}."))?;
	read_entry_data(reader, entry).await
}

/// The bytes of every selected entry. Entries that sit close together come
/// from one range request.
pub async fn extract_entries<R: RangeReader>(
	reader: &R,
	select: impl Fn(&str) -> bool,
) -> Result<Vec<(String, Vec<u8>)>> {
	let mut entries = read_entries(reader).await?;
	entries.sort_by_key(|entry| entry.local_header_offset);
	let mut spans: Vec<(u64, u64, Vec<&ZipEntry>)> = Vec::new();
	for (index, entry) in entries.iter().enumerate() {
		if !select(&entry.name) {
			continue;
		}
		let start = entry.local_header_offset;
		let end = entries
			.get(index + 1)
			.map_or(reader.size(), |next| next.local_header_offset);
		match spans.last_mut() {
			Some(last) if start - last.1 <= MAX_GAP => {
				last.1 = end;
				last.2.push(entry);
			}
			_ => spans.push((start, end, vec![entry])),
		}
	}
	let reads = spans.into_iter().map(|(start, end, entries)| async move {
		let window = Window {
			reader,
			start,
			bytes: reader.read(start, end).await?,
		};
		let mut extracted = Vec::with_capacity(entries.len());
		for entry in entries {
			extracted.push((entry.name.clone(), read_entry_data(&window, entry).await?));
		}
		Ok::<_, String>(extracted)
	});
	let mut extracted = Vec::new();
	for span in futures_util::future::join_all(reads).await {
		extracted.extend(span?);
	}
	Ok(extracted)
}

#[cfg(test)]
pub mod tests {
	use std::io::Write;
	use std::sync::atomic::{AtomicUsize, Ordering};

	use super::*;

	/// Builds a zip archive with stored or deflated entries.
	pub fn build_zip(files: &[(&str, &str, bool)], comment: &str) -> Vec<u8> {
		let mut locals = Vec::new();
		let mut central = Vec::new();
		for (name, text, deflate) in files {
			let raw = text.as_bytes();
			let data = if *deflate {
				let mut encoder =
					flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
				encoder.write_all(raw).unwrap();
				encoder.finish().unwrap()
			} else {
				raw.to_vec()
			};
			let offset = u32::try_from(locals.len()).unwrap();
			let method: u16 = if *deflate { 8 } else { 0 };
			let name_length = u16::try_from(name.len()).unwrap();
			let mut local = vec![0; 30];
			local[0..4].copy_from_slice(&LOCAL_SIGNATURE.to_le_bytes());
			local[8..10].copy_from_slice(&method.to_le_bytes());
			local[26..28].copy_from_slice(&name_length.to_le_bytes());
			locals.extend(local);
			locals.extend(name.as_bytes());
			locals.extend(&data);
			let mut entry = vec![0; 46];
			entry[0..4].copy_from_slice(&ENTRY_SIGNATURE.to_le_bytes());
			entry[10..12].copy_from_slice(&method.to_le_bytes());
			entry[16..20].copy_from_slice(&crc32fast::hash(raw).to_le_bytes());
			entry[20..24].copy_from_slice(&u32::try_from(data.len()).unwrap().to_le_bytes());
			entry[24..28].copy_from_slice(&u32::try_from(raw.len()).unwrap().to_le_bytes());
			entry[28..30].copy_from_slice(&name_length.to_le_bytes());
			entry[42..46].copy_from_slice(&offset.to_le_bytes());
			central.extend(entry);
			central.extend(name.as_bytes());
		}
		let mut end = vec![0; 22];
		end[0..4].copy_from_slice(&END_SIGNATURE.to_le_bytes());
		end[10..12].copy_from_slice(&u16::try_from(files.len()).unwrap().to_le_bytes());
		end[12..16].copy_from_slice(&u32::try_from(central.len()).unwrap().to_le_bytes());
		end[16..20].copy_from_slice(&u32::try_from(locals.len()).unwrap().to_le_bytes());
		end[20..22].copy_from_slice(&u16::try_from(comment.len()).unwrap().to_le_bytes());
		[locals, central, end, comment.as_bytes().to_vec()].concat()
	}

	fn block_on<F: Future>(future: F) -> F::Output {
		tokio::runtime::Builder::new_current_thread()
			.build()
			.unwrap()
			.block_on(future)
	}

	fn text(bytes: &[u8]) -> String {
		String::from_utf8(bytes.to_vec()).unwrap()
	}

	fn archive() -> Vec<u8> {
		build_zip(
			&[
				("RobloxStudioBeta.exe", "binary", false),
				("en-us.json", &r#"{"documentation": ""}"#.repeat(50), true),
			],
			"archive comment",
		)
	}

	#[test]
	fn extracts_deflated_and_stored_entries() {
		let reader = MemoryReader(archive());
		assert_eq!(
			text(&block_on(extract_entry(&reader, "en-us.json")).unwrap()),
			r#"{"documentation": ""}"#.repeat(50)
		);
		assert_eq!(
			text(&block_on(extract_entry(&reader, "RobloxStudioBeta.exe")).unwrap()),
			"binary"
		);
	}

	#[test]
	fn rejects_a_missing_or_corrupted_entry() {
		let missing = block_on(extract_entry(&MemoryReader(archive()), "Missing.xml")).unwrap_err();
		assert!(
			missing.contains("does not contain Missing.xml"),
			"{missing}"
		);
		let mut corrupted = build_zip(&[("a.txt", "hello", false)], "");
		corrupted[30 + "a.txt".len()] = b'j';
		let error = block_on(extract_entry(&MemoryReader(corrupted), "a.txt")).unwrap_err();
		assert!(error.contains("integrity check"), "{error}");
	}

	struct CountingReader {
		inner: MemoryReader,
		reads: AtomicUsize,
	}

	impl RangeReader for CountingReader {
		fn size(&self) -> u64 {
			self.inner.size()
		}

		async fn read(&self, start: u64, end: u64) -> Result<Vec<u8>> {
			self.reads.fetch_add(1, Ordering::Relaxed);
			self.inner.read(start, end).await
		}
	}

	fn icons() -> Vec<u8> {
		build_zip(
			&[
				("Icons\\Part.png", "part", false),
				("Icons\\Part@2x.png", "part 2x", false),
				("Icons\\Model.png", &"model".repeat(20), true),
				("Other\\Readme.txt", "readme", false),
			],
			"",
		)
	}

	#[test]
	fn extracts_every_selected_entry() {
		let entries = block_on(extract_entries(&MemoryReader(icons()), |name| {
			name.starts_with("Icons\\") && !name.contains('@')
		}))
		.unwrap();
		let entries: Vec<(String, String)> = entries
			.into_iter()
			.map(|(name, bytes)| (name, text(&bytes)))
			.collect();
		assert_eq!(
			entries,
			[
				("Icons\\Part.png".to_owned(), "part".to_owned()),
				("Icons\\Model.png".to_owned(), "model".repeat(20))
			]
		);
	}

	#[test]
	fn reads_nearby_entries_with_one_range_request() {
		let counting = |select: fn(&str) -> bool| {
			let reader = CountingReader {
				inner: MemoryReader(icons()),
				reads: AtomicUsize::new(0),
			};
			block_on(extract_entries(&reader, select)).unwrap();
			reader.reads.load(Ordering::Relaxed)
		};
		assert_eq!(
			counting(|name| name.starts_with("Icons\\")) - counting(|_| false),
			1
		);
	}
}
