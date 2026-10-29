//! HTTP requests to Roblox's CDN and APIs.

use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, LAST_MODIFIED, RANGE};
use reqwest::{Client, StatusCode};

use crate::zip::{RangeReader, Result};

pub fn client() -> Result<Client> {
	Client::builder()
		.user_agent(concat!("roblox-api-ref/", env!("CARGO_PKG_VERSION")))
		.build()
		.map_err(|error| error.to_string())
}

pub async fn get_bytes(client: &Client, url: &str) -> Result<Vec<u8>> {
	let response = client
		.get(url)
		.send()
		.await
		.map_err(|error| format!("Failed to download {url}: {error}"))?;
	if !response.status().is_success() {
		return Err(format!(
			"Failed to download {url}: HTTP {}",
			response.status().as_u16()
		));
	}
	response
		.bytes()
		.await
		.map(|bytes| bytes.to_vec())
		.map_err(|error| format!("Failed to download {url}: {error}"))
}

pub async fn get_json(client: &Client, url: &str) -> Result<serde_json::Value> {
	serde_json::from_slice(&get_bytes(client, url).await?)
		.map_err(|error| format!("{url}: {error}"))
}

/// When the server last changed `url`, in milliseconds since the epoch.
pub async fn last_modified(client: &Client, url: &str) -> Option<i64> {
	let response = client.head(url).send().await.ok()?;
	if !response.status().is_success() {
		return None;
	}
	let header = response.headers().get(LAST_MODIFIED)?.to_str().ok()?;
	rapi_core::js::parse_http_date(header)
}

pub struct HttpReader {
	client: Client,
	url: String,
	size: u64,
}

impl HttpReader {
	pub async fn open(client: &Client, url: &str) -> Result<Self> {
		let response = client
			.head(url)
			.send()
			.await
			.map_err(|error| format!("Failed to read {url}: {error}"))?;
		if !response.status().is_success() {
			return Err(format!(
				"Failed to read {url}: HTTP {}",
				response.status().as_u16()
			));
		}
		let headers = response.headers();
		if headers
			.get(ACCEPT_RANGES)
			.and_then(|value| value.to_str().ok())
			!= Some("bytes")
		{
			return Err(format!("{url} does not support byte range requests."));
		}
		let size = headers
			.get(CONTENT_LENGTH)
			.and_then(|value| value.to_str().ok())
			.and_then(|value| value.parse::<u64>().ok())
			.filter(|size| *size > 0)
			.ok_or_else(|| format!("{url} did not report its size."))?;
		Ok(Self {
			client: client.clone(),
			url: url.to_owned(),
			size,
		})
	}
}

impl RangeReader for HttpReader {
	fn size(&self) -> u64 {
		self.size
	}

	async fn read(&self, start: u64, end: u64) -> Result<Vec<u8>> {
		let url = &self.url;
		let response = self
			.client
			.get(url)
			.header(RANGE, format!("bytes={start}-{}", end - 1))
			.send()
			.await
			.map_err(|error| format!("Failed to read {url}: {error}"))?;
		if response.status() != StatusCode::PARTIAL_CONTENT {
			return Err(format!(
				"Expected a partial response from {url}, got HTTP {}",
				response.status().as_u16()
			));
		}
		let bytes = response
			.bytes()
			.await
			.map_err(|error| format!("Failed to read {url}: {error}"))?;
		if bytes.len() as u64 != end - start {
			return Err(format!(
				"Expected {} bytes from {url}, got {}",
				end - start,
				bytes.len()
			));
		}
		Ok(bytes.to_vec())
	}
}
