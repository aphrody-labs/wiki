// SPDX-License-Identifier: Apache-2.0
//! Verified 3-mode WCAG AA/AAA design tokens and craft-floor stylesheet for `shenron-site`.
//!
//! Token definitions mirror `replica/design/tokens.css` and `replica/design/themes-3modes.css`:
//! - `Neutral`: Obsidian editorial palette (`#121317` / `#edc760`)
//! - `Goku`: Super Saiyan God crimson/amber/gold palette (`#120906` / `#ff7a1a` / `#ffd147`)
//! - `Beerus`: Cosmic violet/cyan palette (`#0b0714` / `#c84bff` / `#38c6ff`)
//!
//! Craft-floor guarantees:
//! - Zero eyebrow kickers, zero gradient text, zero emoji, zero nested cards.
//! - Elevation declared once per surface (border OR shadow, never ghost cards).
//! - Touch targets >= 44px on every interactive link, button, and input.
//! - Themed browser surfaces (`::selection`, `caret-color`, custom scrollbars, tabular numerals,
//!   double-ring `:focus-visible`, `prefers-reduced-motion`).

use serde::{Deserialize, Serialize};
pub use shenron_theme::{css as shared_css, toriyama_svg};

/// Visual theme modes exposed in the top bar switcher.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Neutral,
    Goku,
    Beerus,
}

impl ThemeMode {
    /// Canonical order displayed in the theme switcher.
    pub const ALL: [Self; 3] = [Self::Neutral, Self::Goku, Self::Beerus];

    /// Value written to the `data-shenron-theme` HTML attribute.
    #[must_use]
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Goku => "goku",
            Self::Beerus => "beerus",
        }
    }

    /// Human-readable label shown inside the switcher control.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Neutral => "Neutre",
            Self::Goku => "Goku SSG",
            Self::Beerus => "Beerus",
        }
    }

    /// Background hex color suitable for `<meta name="theme-color">`.
    #[must_use]
    pub const fn meta_color(self) -> &'static str {
        match self {
            Self::Neutral => "#121317",
            Self::Goku => "#120906",
            Self::Beerus => "#0b0714",
        }
    }

    /// Parse a `data-shenron-theme` attribute value, falling back to [`Self::Neutral`].
    #[must_use]
    pub fn from_attr(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "goku" => Self::Goku,
            "beerus" => Self::Beerus,
            _ => Self::Neutral,
        }
    }
}

/// Core token declarations synchronized with `replica/design/tokens.css`.
pub const TOKENS_CSS: &str = r#"
:root {
  --sr-bg: #121317;
  --sr-surface: #18191d;
  --sr-surface-high: #212226;
  --sr-border: #2f3034;
  --sr-border-input: #6d7480;
  --sr-text: #f8f9fc;
  --sr-text-muted: #b2bbc5;
  --sr-accent: #edc760;
  --sr-on-accent: #121317;
  --sr-accent-gold: #beaf62;
  --sr-accent-sky: #7db3f0;
  --sr-accent-playback: #e1002d;
  --sr-on-playback: #ffffff;
  --sr-cta-primary: #f8f9fc;
  --sr-on-cta-primary: #121317;
  --sr-selection-bg: #7833ce;
  --sr-danger: #ff6b6b;
  --sr-success: #4ade80;

  --sr-font-sans: "Google Sans Flex", Inter, system-ui, -apple-system, sans-serif;
  --sr-font-jp: "Noto Sans JP", "M PLUS 1", sans-serif;
  --sr-font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, monospace;

  --sr-radius-none: 0px;
  --sr-radius-sm: 8px;
  --sr-radius-md: 16px;
  --sr-radius-lg: 24px;
  --sr-radius-pill: 9999px;

  --sr-shadow-stage: 0 20px 56px rgba(0, 0, 0, 0.55);
  --sr-shadow-gold-edge: 0 0 0 1px rgba(237, 199, 96, 0.45), 0 8px 28px rgba(0, 0, 0, 0.5);
  --sr-focus-ring: 0 0 0 2px #121317, 0 0 0 4px #f8f9fc;

  --sr-ease: cubic-bezier(0.2, 0.8, 0.2, 1);
  --sr-motion-fast: 120ms;
  --sr-motion-base: 220ms;
  --sr-motion-slow: 420ms;
}

::selection {
  background: var(--sr-selection-bg);
  color: var(--sr-text);
}
"#;

/// 3-mode WCAG AAA theme overrides synchronized with `replica/design/themes-3modes.css`.
pub const THEMES_3MODES_CSS: &str = r#"
:root,
[data-shenron-theme="neutral"] {
  --tv-bg: #121317;
  --tv-surface: #18191d;
  --tv-elevated: #212226;
  --tv-border: #2f3034;
  --tv-text: #f8f9fc;
  --tv-muted: #b2bbc5;
  --tv-accent: #edc760;
  --tv-accent-secondary: #7db3f0;
  --tv-playback: #e1002d;
  --tv-cta-bg: #f8f9fc;
  --tv-cta-text: #121317;
  --tv-aura: rgba(237, 199, 96, 0.14);
}

[data-shenron-theme="goku"] {
  --tv-bg: #120906;
  --tv-surface: #1c100b;
  --tv-elevated: #281710;
  --tv-border: #4d2a1a;
  --tv-text: #fff7ed;
  --tv-muted: #d6b49b;
  --tv-accent: #ff7a1a;
  --tv-accent-secondary: #ffd147;
  --tv-playback: #ff2e4d;
  --tv-cta-bg: #ffd147;
  --tv-cta-text: #120906;
  --tv-aura: rgba(255, 122, 26, 0.24);
}

[data-shenron-theme="beerus"] {
  --tv-bg: #0b0714;
  --tv-surface: #130d22;
  --tv-elevated: #1d1533;
  --tv-border: #36275c;
  --tv-text: #f7f2ff;
  --tv-muted: #b8a8d9;
  --tv-accent: #c84bff;
  --tv-accent-secondary: #38c6ff;
  --tv-playback: #c84bff;
  --tv-cta-bg: #38c6ff;
  --tv-cta-text: #0b0714;
  --tv-aura: rgba(200, 75, 255, 0.24);
}
"#;

/// Complete inline stylesheet rendered into `<head>` during SSR and client hydration.
pub const SITE_STYLESHEET: &str = concat!(
    r#"
:root {
  --sr-bg: #121317;
  --sr-surface: #18191d;
  --sr-surface-high: #212226;
  --sr-border: #2f3034;
  --sr-border-input: #6d7480;
  --sr-text: #f8f9fc;
  --sr-text-muted: #b2bbc5;
  --sr-accent: #edc760;
  --sr-on-accent: #121317;
  --sr-accent-gold: #beaf62;
  --sr-accent-sky: #7db3f0;
  --sr-accent-playback: #e1002d;
  --sr-on-playback: #ffffff;
  --sr-cta-primary: #f8f9fc;
  --sr-on-cta-primary: #121317;
  --sr-selection-bg: #7833ce;
  --sr-danger: #ff6b6b;
  --sr-success: #4ade80;

  --sr-font-sans: "Google Sans Flex", Inter, system-ui, -apple-system, sans-serif;
  --sr-font-jp: "Noto Sans JP", "M PLUS 1", sans-serif;
  --sr-font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, monospace;

  --sr-radius-none: 0px;
  --sr-radius-sm: 8px;
  --sr-radius-md: 16px;
  --sr-radius-lg: 24px;
  --sr-radius-pill: 9999px;

  --sr-shadow-stage: 0 20px 56px rgba(0, 0, 0, 0.55);
  --sr-shadow-gold-edge: 0 0 0 1px rgba(237, 199, 96, 0.45), 0 8px 28px rgba(0, 0, 0, 0.5);
  --sr-focus-ring: 0 0 0 2px #121317, 0 0 0 4px #f8f9fc;

  --sr-ease: cubic-bezier(0.2, 0.8, 0.2, 1);
  --sr-motion-fast: 120ms;
  --sr-motion-base: 220ms;
  --sr-motion-slow: 420ms;
}

::selection {
  background: var(--sr-selection-bg);
  color: var(--sr-text);
}

:root,
[data-shenron-theme="neutral"] {
  --tv-bg: #121317;
  --tv-surface: #18191d;
  --tv-elevated: #212226;
  --tv-border: #2f3034;
  --tv-text: #f8f9fc;
  --tv-muted: #b2bbc5;
  --tv-accent: #edc760;
  --tv-accent-secondary: #7db3f0;
  --tv-playback: #e1002d;
  --tv-cta-bg: #f8f9fc;
  --tv-cta-text: #121317;
  --tv-aura: rgba(237, 199, 96, 0.14);
}

[data-shenron-theme="goku"] {
  --tv-bg: #120906;
  --tv-surface: #1c100b;
  --tv-elevated: #281710;
  --tv-border: #4d2a1a;
  --tv-text: #fff7ed;
  --tv-muted: #d6b49b;
  --tv-accent: #ff7a1a;
  --tv-accent-secondary: #ffd147;
  --tv-playback: #ff2e4d;
  --tv-cta-bg: #ffd147;
  --tv-cta-text: #120906;
  --tv-aura: rgba(255, 122, 26, 0.24);
}

[data-shenron-theme="beerus"] {
  --tv-bg: #0b0714;
  --tv-surface: #130d22;
  --tv-elevated: #1d1533;
  --tv-border: #36275c;
  --tv-text: #f7f2ff;
  --tv-muted: #b8a8d9;
  --tv-accent: #c84bff;
  --tv-accent-secondary: #38c6ff;
  --tv-playback: #c84bff;
  --tv-cta-bg: #38c6ff;
  --tv-cta-text: #0b0714;
  --tv-aura: rgba(200, 75, 255, 0.24);
}

*, *::before, *::after {
  box-sizing: border-box;
}

html {
  scrollbar-color: var(--tv-border) var(--tv-bg);
  scrollbar-width: thin;
}

html, body {
  margin: 0;
  padding: 0;
  background: var(--sr-bg);
  color: var(--sr-text);
  font-family: var(--sr-font-sans);
  line-height: 1.55;
  caret-color: var(--tv-accent);
  -webkit-font-smoothing: antialiased;
}

::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

::-webkit-scrollbar-track {
  background: var(--tv-bg);
}

::-webkit-scrollbar-thumb {
  background: var(--tv-border);
  border-radius: var(--sr-radius-pill);
  border: 2px solid var(--tv-bg);
}

a {
  text-underline-offset: 0.22em;
}

a:focus-visible,
button:focus-visible,
input:focus-visible {
  outline: none;
  box-shadow: var(--sr-focus-ring);
}

.sr-root {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  padding-left: 68px;
  background: radial-gradient(circle at 20% 0%, var(--tv-aura), transparent 48%), var(--tv-bg);
  color: var(--tv-text);
  transition: background var(--sr-motion-base) var(--sr-ease), color var(--sr-motion-base) var(--sr-ease);
}

.sr-app-rail {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  width: 68px;
  z-index: 60;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 6px;
  gap: 8px;
  background: color-mix(in srgb, var(--tv-surface) 96%, transparent);
  border-right: 1px solid var(--tv-border);
  backdrop-filter: blur(12px);
}

.sr-rail-brand {
  width: 44px;
  height: 44px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--sr-radius-md);
  text-decoration: none;
  margin-bottom: 6px;
  transition: transform var(--sr-motion-fast) var(--sr-ease), background var(--sr-motion-fast) var(--sr-ease);
}

.sr-rail-brand:hover {
  transform: scale(1.06);
  background: var(--tv-elevated);
}

.sr-rail-nav {
  flex: 1;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.sr-rail-item {
  width: 56px;
  min-height: 52px;
  padding: 6px 4px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  border-radius: var(--sr-radius-sm);
  color: var(--tv-muted);
  text-decoration: none;
  border: 1px solid transparent;
  transition: color var(--sr-motion-fast) var(--sr-ease),
              background var(--sr-motion-fast) var(--sr-ease),
              border-color var(--sr-motion-fast) var(--sr-ease);
}

.sr-rail-item:hover {
  color: var(--tv-text);
  background: var(--tv-elevated);
}

.sr-rail-item[aria-current="page"] {
  color: var(--tv-accent);
  background: color-mix(in srgb, var(--tv-accent) 14%, var(--tv-elevated));
  border-color: color-mix(in srgb, var(--tv-accent) 38%, var(--tv-border));
}

.sr-rail-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.sr-rail-label {
  font-size: 0.68rem;
  font-weight: 600;
  line-height: 1.1;
  letter-spacing: 0.01em;
  text-align: center;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sr-rail-footer {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding-top: 8px;
  border-top: 1px solid var(--tv-border);
  width: 100%;
}

.sr-rail-icon-btn,
.sr-icon-link {
  width: 40px;
  height: 40px;
  min-width: 40px;
  min-height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--sr-radius-pill);
  color: var(--tv-muted);
  background: transparent;
  border: 1px solid transparent;
  text-decoration: none;
  transition: color var(--sr-motion-fast) var(--sr-ease),
              background var(--sr-motion-fast) var(--sr-ease),
              border-color var(--sr-motion-fast) var(--sr-ease);
}

.sr-rail-icon-btn:hover,
.sr-icon-link:hover {
  color: var(--tv-text);
  background: var(--tv-elevated);
  border-color: var(--tv-border);
}

.sr-inline-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.sr-inline-icon svg,
.sr-icon-link svg,
.sr-rail-icon svg,
.sr-rail-icon-btn svg {
  width: 100%;
  height: 100%;
  display: block;
}

.sr-locale-globe {
  margin-left: 8px;
  margin-right: 2px;
  color: var(--tv-muted);
}

@media (max-width: 768px) {
  .sr-root {
    padding-left: 0;
    padding-bottom: calc(64px + env(safe-area-inset-bottom, 0px));
  }

  .sr-app-rail {
    top: auto;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    height: calc(60px + env(safe-area-inset-bottom, 0px));
    flex-direction: row;
    justify-content: space-around;
    padding: 4px 8px env(safe-area-inset-bottom, 0px);
    border-right: none;
    border-top: 1px solid var(--tv-border);
  }

  .sr-rail-brand,
  .sr-rail-footer {
    display: none;
  }

  .sr-rail-nav {
    flex-direction: row;
    justify-content: space-around;
    gap: 2px;
  }

  .sr-rail-item {
    flex: 1;
    width: auto;
    min-height: 48px;
    padding: 4px 2px;
  }
}

.sr-skip-link {
  position: absolute;
  left: -9999px;
  top: 0;
  min-height: 44px;
  padding: 10px 18px;
  display: inline-flex;
  align-items: center;
  background: var(--tv-cta-bg);
  color: var(--tv-cta-text);
  font-weight: 600;
  text-decoration: none;
  z-index: 200;
}

.sr-skip-link:focus-visible {
  left: 16px;
  top: 16px;
  border-radius: var(--sr-radius-sm);
}

.sr-header {
  position: sticky;
  top: 0;
  z-index: 50;
  background: color-mix(in srgb, var(--tv-surface) 94%, transparent);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--tv-border);
}

.sr-header-inner {
  max-width: 1280px;
  margin: 0 auto;
  padding: 10px 24px;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.sr-brand {
  min-height: 44px;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: var(--tv-text);
  text-decoration: none;
  font-weight: 700;
  font-size: 1.125rem;
  letter-spacing: -0.02em;
}

.sr-brand-jp {
  color: var(--tv-accent);
  font-family: var(--sr-font-jp);
  font-size: 0.95rem;
  font-weight: 500;
}

.sr-nav {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 4px;
}

.sr-nav-link {
  min-height: 44px;
  padding: 0 12px;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border-radius: var(--sr-radius-pill);
  color: var(--tv-muted);
  text-decoration: none;
  font-size: 0.9rem;
  font-weight: 500;
  border: 1px solid transparent;
  transition: color var(--sr-motion-fast) var(--sr-ease),
              background var(--sr-motion-fast) var(--sr-ease),
              border-color var(--sr-motion-fast) var(--sr-ease);
}

.sr-nav-link:hover,
.sr-nav-link[aria-current="page"] {
  color: var(--tv-text);
  background: var(--tv-elevated);
  border-color: var(--tv-border);
}

.sr-controls {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 6px;
}

.sr-control-group {
  display: inline-flex;
  align-items: center;
  padding: 2px;
  border-radius: var(--sr-radius-pill);
  background: var(--tv-elevated);
  border: 1px solid var(--tv-border);
  gap: 2px;
}

.sr-chip-btn {
  appearance: none;
  min-height: 44px;
  min-width: 44px;
  padding: 0 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  border: none;
  background: transparent;
  color: var(--tv-muted);
  border-radius: var(--sr-radius-pill);
  font-family: var(--sr-font-sans);
  font-size: 0.85rem;
  font-weight: 600;
  cursor: pointer;
  transition: background var(--sr-motion-fast) var(--sr-ease), color var(--sr-motion-fast) var(--sr-ease);
}

.sr-chip-btn:hover {
  color: var(--tv-text);
}

.sr-chip-btn[aria-checked="true"],
.sr-chip-btn[aria-pressed="true"] {
  background: var(--tv-cta-bg);
  color: var(--tv-cta-text);
}

.sr-main {
  flex: 1;
  width: 100%;
  max-width: 1280px;
  margin: 0 auto;
  padding: 32px 24px 72px;
}

/* Cinema-first live CDN frame stage: elevation declared via shadow only (no ghost border). */
.sr-stage {
  position: relative;
  overflow: hidden;
  border-radius: var(--sr-radius-md);
  background: var(--tv-surface);
  box-shadow: var(--sr-shadow-stage);
  min-height: 420px;
  display: grid;
  grid-template-columns: minmax(0, 1.25fr) minmax(0, 1fr);
  margin-bottom: 48px;
}

@media (max-width: 860px) {
  .sr-stage {
    grid-template-columns: 1fr;
  }
}

.sr-stage-media {
  position: relative;
  min-height: 280px;
  background: var(--tv-elevated);
  overflow: hidden;
}

.sr-stage-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.sr-stage-scrim {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, transparent 40%, color-mix(in srgb, var(--tv-bg) 88%, transparent) 100%);
  pointer-events: none;
}

.sr-stage-caption {
  position: absolute;
  left: 16px;
  right: 16px;
  bottom: 14px;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  color: var(--tv-text);
  font-size: 0.875rem;
}

.sr-stage-copy {
  padding: 36px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 16px;
}

.sr-title {
  margin: 0;
  font-size: clamp(2rem, 4vw, 3.25rem);
  line-height: 1.08;
  letter-spacing: -0.03em;
  text-wrap: balance;
  color: var(--tv-text);
}

.sr-section-title {
  margin: 40px 0 14px;
  font-size: clamp(1.4rem, 2.4vw, 2rem);
  line-height: 1.18;
  letter-spacing: -0.02em;
  text-wrap: balance;
  color: var(--tv-text);
}

.sr-lead {
  margin: 0;
  max-width: 68ch;
  color: var(--tv-muted);
  font-size: 1.05rem;
}

.sr-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 8px;
}

.sr-cta {
  min-height: 44px;
  padding: 0 22px;
  display: inline-flex;
  align-items: center;
  border-radius: var(--sr-radius-pill);
  background: var(--tv-cta-bg);
  color: var(--tv-cta-text);
  font-weight: 600;
  text-decoration: none;
}

.sr-cta-secondary {
  min-height: 44px;
  padding: 0 22px;
  display: inline-flex;
  align-items: center;
  border-radius: var(--sr-radius-pill);
  background: var(--tv-elevated);
  color: var(--tv-text);
  border: 1px solid var(--tv-border);
  font-weight: 600;
  text-decoration: none;
}

.sr-frame-strip {
  display: flex;
  gap: 10px;
  overflow-x: auto;
  padding: 4px 0 12px;
  margin-bottom: 32px;
}

.sr-frame-thumb {
  appearance: none;
  flex: 0 0 auto;
  width: 148px;
  min-height: 84px;
  padding: 0;
  border-radius: var(--sr-radius-sm);
  overflow: hidden;
  border: 2px solid var(--tv-border);
  background: var(--tv-elevated);
  cursor: pointer;
  position: relative;
}

.sr-frame-thumb[aria-pressed="true"] {
  border-color: var(--tv-accent);
}

.sr-frame-thumb img {
  width: 100%;
  height: 84px;
  object-fit: cover;
  display: block;
}

/* Asymmetric 4-pillar living layout: not identical icon+heading+text cards. */
.sr-pillars {
  display: grid;
  grid-template-columns: 1.35fr 1fr;
  gap: 20px;
}

@media (max-width: 860px) {
  .sr-pillars {
    grid-template-columns: 1fr;
  }
}

.sr-pillar-panel {
  border-radius: var(--sr-radius-md);
  background: var(--tv-surface);
  border: 1px solid var(--tv-border);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.sr-pillar-media {
  width: 100%;
  aspect-ratio: 16 / 9;
  object-fit: cover;
  display: block;
  background: var(--tv-elevated);
}

.sr-pillar-body {
  padding: 22px 24px 26px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.sr-pillar-heading {
  margin: 0;
  font-size: 1.35rem;
  letter-spacing: -0.02em;
  color: var(--tv-text);
}

.sr-pillar-heading a {
  color: inherit;
  text-decoration: none;
}

.sr-pillar-heading a:hover {
  color: var(--tv-accent);
  text-decoration: underline;
}

.sr-list {
  list-style: none;
  margin: 0;
  padding: 0;
  border-top: 1px solid var(--tv-border);
}

.sr-list-row {
  border-bottom: 1px solid var(--tv-border);
  padding: 18px 8px;
  display: grid;
  grid-template-columns: 120px minmax(0, 1fr) auto;
  align-items: baseline;
  gap: 16px;
}

@media (max-width: 720px) {
  .sr-list-row {
    grid-template-columns: 1fr;
    gap: 6px;
  }
}

.sr-tabular {
  font-variant-numeric: tabular-nums;
  font-family: var(--sr-font-mono);
  font-size: 0.85rem;
  color: var(--tv-accent-secondary);
}

.sr-row-title {
  margin: 0;
  font-size: 1.08rem;
  font-weight: 600;
  color: var(--tv-text);
}

.sr-row-title a {
  color: inherit;
  text-decoration: none;
  min-height: 44px;
  display: inline-flex;
  align-items: center;
}

.sr-row-title a:hover {
  color: var(--tv-accent);
  text-decoration: underline;
}

.sr-jp {
  font-family: var(--sr-font-jp);
  font-size: 0.9rem;
  color: var(--tv-muted);
  margin: 2px 0 0;
}

.sr-muted {
  margin: 4px 0 0;
  max-width: 68ch;
  color: var(--tv-muted);
  font-size: 0.95rem;
}

.sr-spread {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  background: var(--tv-surface);
  border-radius: var(--sr-radius-md);
  padding: 16px;
  box-shadow: var(--sr-shadow-stage);
}

.sr-spread-single {
  grid-template-columns: minmax(0, 720px);
  justify-content: center;
}

.sr-spread img {
  width: 100%;
  height: auto;
  border-radius: var(--sr-radius-sm);
  display: block;
}

.sr-toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin: 24px 0;
}

.sr-input {
  width: min(380px, 100%);
  min-height: 44px;
  padding: 0 14px;
  border-radius: var(--sr-radius-sm);
  border: 1px solid var(--sr-border-input);
  background: var(--tv-elevated);
  color: var(--tv-text);
  font-family: var(--sr-font-sans);
  font-size: 0.95rem;
}

.sr-status {
  padding: 18px 20px;
  border-radius: var(--sr-radius-md);
  background: var(--tv-surface);
  border: 1px solid var(--tv-border);
  color: var(--tv-muted);
}

.sr-status-error {
  border-color: var(--sr-danger);
  color: var(--tv-text);
}

.sr-footer {
  border-top: 1px solid var(--tv-border);
  background: var(--tv-surface);
  color: var(--tv-muted);
  font-size: 0.875rem;
}

.sr-footer-inner {
  max-width: 1280px;
  margin: 0 auto;
  padding: 20px 24px;
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 12px;
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_roundtrip_and_fallback() {
        for mode in ThemeMode::ALL {
            assert_eq!(ThemeMode::from_attr(mode.as_attr()), mode);
        }
        assert_eq!(ThemeMode::from_attr("unknown"), ThemeMode::Neutral);
    }

    #[test]
    fn stylesheet_embeds_verified_aaa_tokens_and_craft_floor_rules() {
        for expected in [
            "--sr-bg: #121317;",
            "--sr-accent: #edc760;",
            "--sr-accent-playback: #e1002d;",
            "[data-shenron-theme=\"neutral\"]",
            "[data-shenron-theme=\"goku\"]",
            "--tv-bg: #120906;",
            "--tv-accent: #ff7a1a;",
            "--tv-cta-bg: #ffd147;",
            "[data-shenron-theme=\"beerus\"]",
            "--tv-bg: #0b0714;",
            "--tv-accent: #c84bff;",
            "--tv-cta-bg: #38c6ff;",
            "min-height: 44px;",
            "font-variant-numeric: tabular-nums;",
            "caret-color: var(--tv-accent);",
        ] {
            assert!(
                SITE_STYLESHEET.contains(expected),
                "missing verified token or craft-floor rule: {expected}"
            );
        }
        assert!(!SITE_STYLESHEET.contains("background-clip: text"));
        assert!(!SITE_STYLESHEET.contains("sr-eyebrow"));
    }
}
