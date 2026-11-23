// SPDX-License-Identifier: Apache-2.0
//! Pure-Rust manga and databook source registry and HTML/JSON extractors.

use serde::{Deserialize, Serialize};

/// Upstream source provider kind for Dragon Ball manga and databooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MangaSourceKind {
    Jmanga,
    MangaDex,
    OpenBd,
    Shueisha,
    DragonBallFrCdn,
    LocalScan,
}

/// Canonical descriptor for a configured manga or databook source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDescriptor {
    pub id: &'static str,
    pub kind: MangaSourceKind,
    pub series_code: &'static str,
    pub language: &'static str,
    pub base_url: &'static str,
    pub editorial_freeze_applies: bool,
}

/// Verified upstream sources for Dragon Ball manga chapters, covers, and databook metadata.
pub const DEFAULT_MANGA_SOURCES: &[SourceDescriptor] = &[
    SourceDescriptor {
        id: "jmanga-dbs-ja-raw",
        kind: MangaSourceKind::Jmanga,
        series_code: "DBS",
        language: "ja",
        base_url: "https://jmanga.coach/read/%E3%83%89%E3%83%A9%E3%82%B4%E3%83%B3%E3%83%9C%E3%83%BC%E3%83%AB%E8%B6%85/ja/chapter-101-raw/",
        editorial_freeze_applies: false,
    },
    SourceDescriptor {
        id: "mangadex-covers-ja",
        kind: MangaSourceKind::MangaDex,
        series_code: "ALL",
        language: "ja",
        base_url: "https://api.mangadex.org/cover",
        editorial_freeze_applies: false,
    },
    SourceDescriptor {
        id: "openbd-isbn-jp",
        kind: MangaSourceKind::OpenBd,
        series_code: "ALL",
        language: "ja",
        base_url: "https://api.openbd.jp/v1/get",
        editorial_freeze_applies: false,
    },
    SourceDescriptor {
        id: "dbfr-cdn-databooks",
        kind: MangaSourceKind::DragonBallFrCdn,
        series_code: "DATABOOKS",
        language: "ja",
        base_url: "https://cdn.dragonballfr.com/assets/ext",
        editorial_freeze_applies: true,
    },
];

/// Extracts image URLs from a manga reader HTML document (`data-src` or `src` attributes).
#[must_use]
pub fn extract_reader_image_urls(html: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for attr in ["data-src=\"", "data-original=\"", "src=\""] {
        let mut rest = html;
        while let Some(idx) = rest.find(attr) {
            let after = &rest[idx + attr.len()..];
            let Some(end) = after.find('"') else {
                break;
            };
            let candidate = after[..end].trim();
            if is_manga_page_image_url(candidate) && !urls.iter().any(|u| u == candidate) {
                urls.push(candidate.to_owned());
            }
            rest = &after[end + 1..];
        }
    }
    urls
}

fn is_manga_page_image_url(url: &str) -> bool {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return false;
    }
    let lower = url.to_ascii_lowercase();
    let path_only = lower.split('?').next().unwrap_or(&lower);
    let has_img_ext = path_only.ends_with(".jpg")
        || path_only.ends_with(".jpeg")
        || path_only.ends_with(".png")
        || path_only.ends_with(".webp");
    has_img_ext
        && !lower.contains("logo")
        && !lower.contains("avatar")
        && !lower.contains("favicon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sources_include_jmanga_dbs_and_databook_freeze() {
        assert!(DEFAULT_MANGA_SOURCES.iter().any(|s| {
            s.id == "jmanga-dbs-ja-raw" && s.base_url.contains("chapter-101-raw")
        }));
        let db = DEFAULT_MANGA_SOURCES
            .iter()
            .find(|s| s.id == "dbfr-cdn-databooks")
            .expect("databooks source registered");
        assert!(db.editorial_freeze_applies);
    }

    #[test]
    fn extracts_deduplicated_manga_page_images_and_skips_logos() {
        let html = r#"
          <img src="https://jmanga.coach/assets/logo.png" />
          <img data-src="https://img.example.com/dbs/101/001.jpg?v=1" />
          <img data-src="https://img.example.com/dbs/101/002.webp" />
          <img src="https://img.example.com/dbs/101/001.jpg?v=1" />
        "#;
        let urls = extract_reader_image_urls(html);
        assert_eq!(
            urls,
            vec![
                "https://img.example.com/dbs/101/001.jpg?v=1",
                "https://img.example.com/dbs/101/002.webp",
            ]
        );
    }
}
