// SPDX-License-Identifier: Apache-2.0
//! Pure HTTP client targeting `SHENRON_API_URL` (and optional `SHENRON_TV_URL`).
//!
//! No database driver or direct storage access is permitted inside `shenron-site`: every page
//! reads strictly through the public Shenron HTTP contract, including real-time CDN frames from
//! `GET ${SHENRON_API_URL}/api/v1/frames/live`.

#[cfg(feature = "server")]
use dioxus::prelude::dioxus_server;
use dioxus::prelude::{ServerFnError, dioxus_fullstack, server};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use thiserror::Error;
use url::Url;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

fn deserialize_opt_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match val {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(s)) => Some(s),
        Some(serde_json::Value::Number(n)) => Some(n.to_string()),
        Some(other) => Some(other.to_string()),
    })
}

/// Default upstream Shenron API origin used when `SHENRON_API_URL` is not set.
pub const DEFAULT_API_URL: &str = "http://127.0.0.1:3000";

/// Default upstream `shenron-tv` origin used when `SHENRON_TV_URL` is not set.
pub const DEFAULT_TV_URL: &str = "http://127.0.0.1:8793";

/// Typed error returned by [`ApiClient`].
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid API base URL: {0}")]
    InvalidBaseUrl(String),
    #[error("invalid path parameter: {0}")]
    InvalidParam(String),
    #[error("HTTP request failed for {path}: {source}")]
    Transport {
        path: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("HTTP {status} from {path}")]
    HttpStatus { status: u16, path: String },
    #[error("failed to decode JSON from {path}: {source}")]
    Decode {
        path: String,
        #[source]
        source: reqwest::Error,
    },
}

/// Response returned by `GET /api/health`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Real-time CDN frame descriptor returned by `GET /api/v1/frames/live`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LiveFrame {
    #[serde(default)]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default", alias = "src", alias = "image")]
    pub url: String,
    #[serde(default, alias = "mirrorUrl")]
    pub mirror_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_default", alias = "category", alias = "kind")]
    pub pillar: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default)]
    pub era: Option<String>,
    #[serde(default, alias = "episodeNumber")]
    pub episode_number: Option<u32>,
    #[serde(default, alias = "timecodeSec")]
    pub timecode_sec: Option<f64>,
    #[serde(default, alias = "aspectRatio")]
    pub aspect_ratio: Option<String>,
    #[serde(default, alias = "isNotable")]
    pub is_notable: bool,
    #[serde(default)]
    pub href: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default, alias = "dominantColor", alias = "aura")]
    pub dominant_color: Option<String>,
    #[serde(default, alias = "accentColor")]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default, alias = "capturedAt")]
    pub captured_at: Option<String>,
}

impl LiveFrame {
    /// Accessible alternative text for `<img>` rendering.
    #[must_use]
    pub fn alt_text(&self) -> &str {
        if let Some(alt) = self.alt.as_deref().filter(|s| !s.trim().is_empty()) {
            return alt;
        }
        if !self.title.trim().is_empty() {
            return self.title.as_str();
        }
        "Dragon Ball CDN frame"
    }

    /// Validated CSS hex color (strict `#RGB` or `#RRGGBB`) for safe ambient stage styling.
    #[must_use]
    pub fn safe_aura_hex(&self) -> Option<&str> {
        let candidate =
            self.dominant_color.as_deref().or(self.accent_color.as_deref()).map(str::trim)?;
        if is_safe_hex_color(candidate) { Some(candidate) } else { None }
    }
}

/// Envelope returned by `GET /api/v1/frames/live`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LiveFramesResponse {
    #[serde(default, alias = "items")]
    pub frames: Vec<LiveFrame>,
    #[serde(default)]
    pub total: usize,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default, alias = "generatedAt", alias = "updatedAt")]
    pub updated_at: Option<String>,
}

/// Paginated wiki list returned by `GET /api/public/wiki/{category}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiListPage<T> {
    pub category: String,
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(default)]
    pub total: usize,
    #[serde(default)]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

/// Public episode entry from `GET /api/v1/episodes` or `/api/v1/episodes/{slug}`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EpisodeRecord {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default, alias = "canonicalSlug")]
    pub canonical_slug: Option<String>,
    #[serde(default, alias = "numberInSeries", alias = "episode_number")]
    pub number: Option<i64>,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default)]
    pub saga: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_default", alias = "title_fr")]
    pub title: String,
    #[serde(default, alias = "titleJa", alias = "title_ja")]
    pub title_ja: Option<String>,
    #[serde(default, deserialize_with = "deserialize_opt_string_or_int", alias = "airDate", alias = "air_date")]
    pub air_date: Option<String>,
    #[serde(default, alias = "durationSec")]
    pub duration_sec: Option<u32>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default, alias = "frameCount")]
    pub frame_count: usize,
    #[serde(default)]
    pub playable: bool,
    #[serde(default)]
    pub synopsis: Option<String>,
}

impl EpisodeRecord {
    /// Deterministic URL slug used by `/wiki/episodes/:slug`.
    #[must_use]
    pub fn route_slug(&self) -> String {
        if let Some(slug) = self.slug.as_deref().filter(|s| !s.trim().is_empty()) {
            return slug.to_owned();
        }
        if let Some(canonical) = self.canonical_slug.as_deref().filter(|s| !s.trim().is_empty()) {
            return canonical.to_owned();
        }
        self.id.to_string()
    }
}

/// Public movie entry from `GET /api/v1/movies`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MovieRecord {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_default", alias = "title_fr")]
    pub title: String,
    #[serde(default, alias = "titleJa", alias = "title_ja")]
    pub title_ja: Option<String>,
    #[serde(default, deserialize_with = "deserialize_opt_string_or_int", alias = "releaseDate", alias = "release_date")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default)]
    pub synopsis: Option<String>,
}

/// Timeline event from `GET /api/v1/wiki/timeline/events`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineEvent {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub age_year: i64,
    #[serde(default)]
    pub age_month: Option<i64>,
    #[serde(default)]
    pub age_day: Option<i64>,
    #[serde(default)]
    pub era: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub title_ja: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub source_ref: Option<String>,
    #[serde(default)]
    pub canon: bool,
    #[serde(default)]
    pub order_idx: i64,
}

/// Envelope returned by `GET /api/v1/wiki/timeline/events`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineEventsResponse {
    #[serde(default)]
    pub events: Vec<TimelineEvent>,
}

/// Video entry from `shenron-tv` (`GET /api/v1/videos`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TvVideo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
}

/// Envelope returned by `GET /api/v1/videos`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TvVideosResponse {
    #[serde(default)]
    pub videos: Vec<TvVideo>,
}

/// Stateless HTTP client for Shenron public API endpoints.
#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    http: reqwest::Client,
}

impl ApiClient {
    /// Construct a client targeting `base_url` (must be a valid `http://` or `https://` origin).
    pub fn new(base_url: impl Into<String>) -> Result<Self, ApiError> {
        let raw = base_url.into();
        let trimmed = raw.trim().trim_end_matches('/');
        let parsed =
            Url::parse(trimmed).map_err(|_| ApiError::InvalidBaseUrl(trimmed.to_owned()))?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err(ApiError::InvalidBaseUrl(trimmed.to_owned()));
        }

        #[cfg(feature = "server")]
        let _ = rustls::crypto::ring::default_provider().install_default();

        #[cfg(not(target_arch = "wasm32"))]
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(concat!("shenron-site/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|source| ApiError::Transport { path: "/".to_owned(), source })?;

        #[cfg(target_arch = "wasm32")]
        let http = reqwest::Client::new();

        Ok(Self { base_url: trimmed.to_owned(), http })
    }

    /// Build a client from `SHENRON_API_URL` (server-side default `http://127.0.0.1:3000`).
    pub fn from_env() -> Result<Self, ApiError> {
        Self::new(resolve_api_base_url())
    }

    /// Build a client targeting `SHENRON_TV_URL` (falling back to `DEFAULT_TV_URL`).
    pub fn tv_from_env() -> Result<Self, ApiError> {
        Self::new(resolve_tv_base_url())
    }

    /// Return the normalized base URL configured for this client.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Join a relative API path onto this client's base URL.
    #[must_use]
    pub fn url_for(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{path}", self.base_url)
        } else {
            format!("{}/{path}", self.base_url)
        }
    }

    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = self.url_for(path);
        let response = self
            .http
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|source| ApiError::Transport { path: path.to_owned(), source })?;

        let status = response.status();
        if !status.is_success() {
            return Err(ApiError::HttpStatus { status: status.as_u16(), path: path.to_owned() });
        }

        response
            .json::<T>()
            .await
            .map_err(|source| ApiError::Decode { path: path.to_owned(), source })
    }

    /// `GET /api/health`
    pub async fn health(&self) -> Result<HealthResponse, ApiError> {
        self.get_json("/api/health").await
    }

    /// `GET /api/v1/frames/live?pillar=&limit=`
    pub async fn list_live_frames(
        &self,
        pillar: Option<&str>,
        limit: usize,
    ) -> Result<LiveFramesResponse, ApiError> {
        let clamped_limit = limit.clamp(1, 64);
        let mut path = format!("/api/v1/frames/live?limit={clamped_limit}");
        if let Some(p) = pillar.map(str::trim).filter(|s| !s.is_empty()) {
            path.push_str("&pillar=");
            path.push_str(&encode_uri_component(p));
        }
        self.get_json(&path).await
    }

    /// `GET /api/public/wiki/episodes?limit=&offset=`
    pub async fn list_episodes(
        &self,
        limit: usize,
        offset: usize,
        query: Option<&str>,
    ) -> Result<WikiListPage<EpisodeRecord>, ApiError> {
        let clamped_limit = limit.clamp(1, 200);
        let mut path = format!("/api/public/wiki/episodes?limit={clamped_limit}&offset={offset}");
        if let Some(q) = query.map(str::trim).filter(|s| !s.is_empty()) {
            path.push_str("&q=");
            path.push_str(&encode_uri_component(q));
        }
        self.get_json(&path).await
    }

    /// `GET /api/public/wiki/episodes/{idOrSlug}`
    pub async fn get_episode(&self, slug: &str) -> Result<EpisodeRecord, ApiError> {
        let clean = validate_path_segment(slug)?;
        let path = format!("/api/public/wiki/episodes/{clean}");
        self.get_json(&path).await
    }

    /// `GET /api/public/wiki/movies?limit=&offset=`
    pub async fn list_movies(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<WikiListPage<MovieRecord>, ApiError> {
        let clamped_limit = limit.clamp(1, 200);
        let path = format!("/api/public/wiki/movies?limit={clamped_limit}&offset={offset}");
        self.get_json(&path).await
    }

    /// `GET /api/v1/wiki/timeline/events`
    pub async fn list_timeline_events(
        &self,
        era: Option<&str>,
    ) -> Result<TimelineEventsResponse, ApiError> {
        let path = match era.map(str::trim).filter(|s| !s.is_empty()) {
            Some(selected) => {
                format!("/api/v1/wiki/timeline/events?era={}", encode_uri_component(selected))
            },
            None => "/api/v1/wiki/timeline/events".to_owned(),
        };
        self.get_json(&path).await
    }

    /// `GET /api/v1/videos`
    pub async fn list_tv_videos(&self) -> Result<TvVideosResponse, ApiError> {
        self.get_json("/api/v1/videos").await
    }
}

/// Resolve `SHENRON_API_URL` from the process environment (server) or fallback default.
#[must_use]
pub fn resolve_api_base_url() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(url) = std::env::var("SHENRON_API_URL") {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return trimmed.to_owned();
        }
    }
    DEFAULT_API_URL.to_owned()
}

/// Resolve `SHENRON_TV_URL` from the process environment (server) or fallback default.
#[must_use]
pub fn resolve_tv_base_url() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(url) = std::env::var("SHENRON_TV_URL") {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return trimmed.to_owned();
        }
    }
    DEFAULT_TV_URL.to_owned()
}

/// Validate a single URL path segment against traversal and encode non-unreserved bytes.
pub fn validate_path_segment(raw: &str) -> Result<String, ApiError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > 200 || trimmed == "." || trimmed == ".." {
        return Err(ApiError::InvalidParam(raw.to_owned()));
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains('\0') {
        return Err(ApiError::InvalidParam(raw.to_owned()));
    }
    Ok(encode_uri_component(trimmed))
}

/// Check that a CSS hex color string contains only `#RGB` or `#RRGGBB` characters.
#[must_use]
pub fn is_safe_hex_color(raw: &str) -> bool {
    let Some(hex) = raw.strip_prefix('#') else {
        return false;
    };
    matches!(hex.len(), 3 | 6) && hex.bytes().all(|b| b.is_ascii_hexdigit())
}

/// RFC 3986 percent-encoding for query values and path segments.
#[must_use]
pub fn encode_uri_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.as_bytes() {
        let unreserved = byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~');
        if unreserved {
            out.push(char::from(*byte));
        } else {
            out.push('%');
            out.push(hex_nibble(byte >> 4));
            out.push(hex_nibble(byte & 0x0f));
        }
    }
    out
}

const fn hex_nibble(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        10..=15 => (b'A' + (nibble - 10)) as char,
        _ => '0',
    }
}

#[cfg(feature = "server")]
fn map_server_error(err: ApiError) -> ServerFnError {
    ServerFnError::new(err.to_string())
}

/// Fullstack server loader: fetch real-time CDN frames from `${SHENRON_API_URL}/api/v1/frames/live`.
#[server]
pub async fn load_live_frames(
    pillar: Option<String>,
    limit: usize,
) -> Result<LiveFramesResponse, ServerFnError> {
    let client = ApiClient::from_env().map_err(map_server_error)?;
    client.list_live_frames(pillar.as_deref(), limit).await.map_err(map_server_error)
}

/// Fullstack server loader: fetch episodes page from `SHENRON_API_URL`.
#[server]
pub async fn load_episodes(
    limit: usize,
    offset: usize,
    query: Option<String>,
) -> Result<WikiListPage<EpisodeRecord>, ServerFnError> {
    let client = ApiClient::from_env().map_err(map_server_error)?;
    client.list_episodes(limit, offset, query.as_deref()).await.map_err(map_server_error)
}

/// Fullstack server loader: fetch one episode by slug or numeric ID from `SHENRON_API_URL`.
#[server]
pub async fn load_episode_detail(slug: String) -> Result<EpisodeRecord, ServerFnError> {
    let client = ApiClient::from_env().map_err(map_server_error)?;
    client.get_episode(&slug).await.map_err(map_server_error)
}

/// Fullstack server loader: fetch movies page from `SHENRON_API_URL`.
#[server]
pub async fn load_movies(
    limit: usize,
    offset: usize,
) -> Result<WikiListPage<MovieRecord>, ServerFnError> {
    let client = ApiClient::from_env().map_err(map_server_error)?;
    client.list_movies(limit, offset).await.map_err(map_server_error)
}

/// Fullstack server loader: fetch timeline events from `SHENRON_API_URL`.
#[server]
pub async fn load_timeline_events(
    era: Option<String>,
) -> Result<TimelineEventsResponse, ServerFnError> {
    let client = ApiClient::from_env().map_err(map_server_error)?;
    client.list_timeline_events(era.as_deref()).await.map_err(map_server_error)
}

/// Fullstack server loader: fetch video catalog from `SHENRON_TV_URL`.
#[server]
pub async fn load_tv_videos() -> Result<TvVideosResponse, ServerFnError> {
    let client = ApiClient::tv_from_env().map_err(map_server_error)?;
    client.list_tv_videos().await.map_err(map_server_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_client_rejects_non_http_origins() {
        assert!(ApiClient::new("file:///etc/passwd").is_err());
        assert!(ApiClient::new("").is_err());
        let ok = ApiClient::new("https://dragonballfr.com/").unwrap();
        assert_eq!(ok.base_url(), "https://dragonballfr.com");
        assert_eq!(
            ok.url_for("/api/v1/frames/live?limit=12"),
            "https://dragonballfr.com/api/v1/frames/live?limit=12"
        );
    }

    #[test]
    fn path_segment_validation_blocks_traversal() {
        assert!(validate_path_segment("..").is_err());
        assert!(validate_path_segment("a/b").is_err());
        assert!(validate_path_segment("").is_err());
        assert_eq!(
            validate_path_segment("1-le-secret-des-dragon-balls").unwrap(),
            "1-le-secret-des-dragon-balls"
        );
        assert_eq!(encode_uri_component("Son Gokû"), "Son%20Gok%C3%BB");
    }

    #[test]
    fn live_frame_validates_hex_colors_and_aliases() {
        let json = r##"{
            "frames": [{
                "id": "f-001",
                "src": "https://cdn.dragonballfr.com/frames/dbz-095.webp",
                "pillar": "anime",
                "title": "Namek — Éveil Super Saiyan",
                "dominantColor": "#ff7a1a"
            }]
        }"##;
        let parsed: LiveFramesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.frames.len(), 1);
        assert_eq!(parsed.frames[0].safe_aura_hex(), Some("#ff7a1a"));
        assert!(!is_safe_hex_color("javascript:alert(1)"));
        assert!(!is_safe_hex_color("#12345"));
    }

    #[test]
    fn episode_record_route_slug_prefers_slug_over_id() {
        let with_slug = EpisodeRecord {
            id: 1,
            slug: Some("1-bulmas-journey".to_owned()),
            title: "Le secret".to_owned(),
            ..EpisodeRecord::default()
        };
        assert_eq!(with_slug.route_slug(), "1-bulmas-journey");

        let id_only = EpisodeRecord {
            id: 42,
            slug: None,
            title: "Le mystère".to_owned(),
            ..EpisodeRecord::default()
        };
        assert_eq!(id_only.route_slug(), "42");
    }

    #[test]
    fn decodes_camel_case_server_payloads_without_drift() {
        let frames_json = r##"{
            "frames": [{
                "id": "dbs-beerus-1-0",
                "kind": "episode",
                "title": "Éveil",
                "subtitle": "Beerus",
                "series": "DBS_BEERUS",
                "era": "Super",
                "episodeNumber": 1,
                "timecodeSec": 142,
                "url": "https://cdn.dragonballfr.com/a.jpg",
                "mirrorUrl": "https://cdn.dragonballfr.com/m.jpg",
                "width": 1920,
                "height": 1080,
                "aspectRatio": "16:9",
                "href": "/tv/dbs-beerus-001",
                "isNotable": true,
                "source": "shenron-cdn"
            }],
            "total": 1,
            "series": "DBS_BEERUS",
            "generatedAt": "2026-11-23T12:00:00.000Z"
        }"##;
        let frames: LiveFramesResponse = serde_json::from_str(frames_json).unwrap();
        assert_eq!(frames.total, 1);
        assert_eq!(frames.frames[0].pillar, "episode");
        assert_eq!(frames.frames[0].episode_number, Some(1));
        assert!(frames.frames[0].is_notable);
        assert_eq!(frames.updated_at.as_deref(), Some("2026-11-23T12:00:00.000Z"));

        let ep_json = r##"{
            "id": 1,
            "slug": null,
            "canonicalSlug": "dbs-beerus-001",
            "series": "DBS_BEERUS",
            "numberInSeries": 1,
            "title": null,
            "titleJa": "破壊神",
            "airDate": 1791590400,
            "durationSec": 1440,
            "frameCount": 3,
            "playable": true
        }"##;
        let ep: EpisodeRecord = serde_json::from_str(ep_json).unwrap();
        assert_eq!(ep.route_slug(), "dbs-beerus-001");
        assert_eq!(ep.number, Some(1));
        assert_eq!(ep.air_date.as_deref(), Some("1791590400"));
        assert!(ep.playable);
    }
}
