// SPDX-License-Identifier: Apache-2.0
//! WCAG AAA 3-mode CSS custom properties (`neutral`, `goku`, `beerus`) shared by
//! `shenron-site` (Dioxus 0.8 SSR) and `shenron-admin-ui`.

/// Complete CSS custom property block for all three verified Shenron themes.
pub const SHENRON_THEMES_CSS: &str = r#":root,
[data-shenron-theme="neutral"],
[data-episode-theme="neutral"] {
  --tv-bg: #0c0d11;
  --tv-surface: #15171e;
  --tv-elevated: #1e212b;
  --tv-border: rgba(255, 255, 255, 0.14);
  --tv-text: #f5f6fa;
  --tv-muted: #b4bac8;
  --tv-accent: #f85b1a;
  --tv-playback: #e53935;
}

[data-shenron-theme="goku"],
[data-episode-theme="goku"] {
  --tv-bg: #fdfae3;
  --tv-surface: #f7f1cd;
  --tv-elevated: #efe6b6;
  --tv-border: rgba(84, 84, 0, 0.28);
  --tv-text: #1c1a12;
  --tv-muted: #4b4628;
  --tv-accent: #c93d08;
  --tv-playback: #c62828;
}

[data-shenron-theme="beerus"],
[data-episode-theme="beerus"] {
  --tv-bg: #0b0714;
  --tv-surface: #150e26;
  --tv-elevated: #21163a;
  --tv-border: rgba(203, 166, 247, 0.24);
  --tv-text: #f4eeff;
  --tv-muted: #c2b2dc;
  --tv-accent: #b86bff;
  --tv-playback: #e040fb;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defines_all_three_wcag_modes() {
        assert!(SHENRON_THEMES_CSS.contains(r#"[data-shenron-theme="neutral"]"#));
        assert!(SHENRON_THEMES_CSS.contains(r#"[data-shenron-theme="goku"]"#));
        assert!(SHENRON_THEMES_CSS.contains(r#"[data-shenron-theme="beerus"]"#));
        assert!(SHENRON_THEMES_CSS.contains("#fdfae3"));
    }
}
