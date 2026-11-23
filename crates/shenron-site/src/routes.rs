// SPDX-License-Identifier: Apache-2.0
//! Application routes and BCP 47 locale handling for `shenron-site`.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::{
    EpisodeDetail, Episodes, Games, Home, LocaleEpisodeDetail, LocaleEpisodes, LocaleGames,
    LocaleHome, LocaleManga, LocaleMovies, LocaleTimeline, LocaleTv, Manga, Movies, NotFound,
    SiteShell, Timeline, Tv,
};

/// Supported BCP 47 locales for Dragon Ball FR.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    Fr,
    En,
    Ja,
}

impl Locale {
    /// Ordered locales exposed in the top bar locale selector.
    pub const ALL: [Self; 3] = [Self::Fr, Self::En, Self::Ja];

    /// Short URL segment (`fr`, `en`, `ja`).
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Fr => "fr",
            Self::En => "en",
            Self::Ja => "ja",
        }
    }

    /// Canonical BCP 47 tag written to `<html lang>` / root `lang`.
    #[must_use]
    pub const fn bcp47(self) -> &'static str {
        match self {
            Self::Fr => "fr-FR",
            Self::En => "en-US",
            Self::Ja => "ja-JP",
        }
    }

    /// Accessible short label shown in the switcher.
    #[must_use]
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Fr => "FR",
            Self::En => "EN",
            Self::Ja => "JA",
        }
    }

    /// Parse a BCP 47 language tag (e.g. `"fr"`, `"fr-FR"`, `"en-GB"`, `"ja"`).
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        let primary = raw.trim().split(['-', '_']).next()?;
        match primary.to_ascii_lowercase().as_str() {
            "fr" => Some(Self::Fr),
            "en" => Some(Self::En),
            "ja" | "jp" => Some(Self::Ja),
            _ => None,
        }
    }

    /// Parse a locale segment, defaulting to French (`fr-FR`) when unrecognized.
    #[must_use]
    pub fn parse_or_default(raw: &str) -> Self {
        Self::parse(raw).unwrap_or(Self::Fr)
    }
}

/// Complete client and SSR route table for `shenron-site`.
///
/// Static routes across the 4 living pillars (Anime Theater `/tv`, Manga Reader `/wiki/manga`,
/// Native Games `/games`, Wiki Encyclopedia `/wiki/episodes`, `/wiki/movies`, `/wiki/timeline`)
/// are declared before `/:lang` so Dioxus Router resolves fixed paths unambiguously alongside
/// BCP 47 locale-prefixed paths.
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SiteShell)]
        #[route("/")]
        Home {},

        #[route("/tv")]
        Tv {},

        #[route("/wiki/manga")]
        Manga {},

        #[route("/games")]
        Games {},

        #[route("/wiki/episodes")]
        Episodes {},

        #[route("/wiki/episodes/:slug")]
        EpisodeDetail { slug: String },

        #[route("/wiki/movies")]
        Movies {},

        #[route("/wiki/timeline")]
        Timeline {},

        #[route("/:lang")]
        LocaleHome { lang: String },

        #[route("/:lang/tv")]
        LocaleTv { lang: String },

        #[route("/:lang/wiki/manga")]
        LocaleManga { lang: String },

        #[route("/:lang/games")]
        LocaleGames { lang: String },

        #[route("/:lang/wiki/episodes")]
        LocaleEpisodes { lang: String },

        #[route("/:lang/wiki/episodes/:slug")]
        LocaleEpisodeDetail { lang: String, slug: String },

        #[route("/:lang/wiki/movies")]
        LocaleMovies { lang: String },

        #[route("/:lang/wiki/timeline")]
        LocaleTimeline { lang: String },

        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

impl Route {
    /// Extract the explicit locale segment if the current route carries one.
    #[must_use]
    pub fn locale(&self) -> Option<Locale> {
        match self {
            Self::LocaleHome { lang }
            | Self::LocaleTv { lang }
            | Self::LocaleManga { lang }
            | Self::LocaleGames { lang }
            | Self::LocaleEpisodes { lang }
            | Self::LocaleEpisodeDetail { lang, .. }
            | Self::LocaleMovies { lang }
            | Self::LocaleTimeline { lang } => Locale::parse(lang),
            _ => None,
        }
    }

    /// Return the equivalent route switched to `target` BCP 47 locale.
    #[must_use]
    pub fn with_locale(&self, target: Locale) -> Self {
        let lang = target.code().to_owned();
        match self {
            Self::Home {} | Self::LocaleHome { .. } | Self::NotFound { .. } => {
                Self::LocaleHome { lang }
            },
            Self::Tv {} | Self::LocaleTv { .. } => Self::LocaleTv { lang },
            Self::Manga {} | Self::LocaleManga { .. } => Self::LocaleManga { lang },
            Self::Games {} | Self::LocaleGames { .. } => Self::LocaleGames { lang },
            Self::Episodes {} | Self::LocaleEpisodes { .. } => Self::LocaleEpisodes { lang },
            Self::EpisodeDetail { slug } | Self::LocaleEpisodeDetail { slug, .. } => {
                Self::LocaleEpisodeDetail { lang, slug: slug.clone() }
            },
            Self::Movies {} | Self::LocaleMovies { .. } => Self::LocaleMovies { lang },
            Self::Timeline {} | Self::LocaleTimeline { .. } => Self::LocaleTimeline { lang },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_parsing_handles_bcp47_tags() {
        assert_eq!(Locale::parse("fr"), Some(Locale::Fr));
        assert_eq!(Locale::parse("fr-FR"), Some(Locale::Fr));
        assert_eq!(Locale::parse("en-US"), Some(Locale::En));
        assert_eq!(Locale::parse("ja-JP"), Some(Locale::Ja));
        assert_eq!(Locale::parse("de"), None);
        assert_eq!(Locale::parse_or_default("de"), Locale::Fr);
    }

    #[test]
    fn route_locale_switching_preserves_slug_and_pillars() {
        let ep = Route::EpisodeDetail { slug: "1-bulmas-journey".to_owned() };
        assert_eq!(
            ep.with_locale(Locale::En),
            Route::LocaleEpisodeDetail {
                lang: "en".to_owned(),
                slug: "1-bulmas-journey".to_owned(),
            }
        );
        assert_eq!(
            Route::Games {}.with_locale(Locale::Ja),
            Route::LocaleGames { lang: "ja".to_owned() }
        );
        assert_eq!(
            Route::Manga {}.with_locale(Locale::Fr),
            Route::LocaleManga { lang: "fr".to_owned() }
        );
    }
}
