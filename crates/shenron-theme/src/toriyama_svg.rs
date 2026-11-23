// SPDX-License-Identifier: Apache-2.0
//! Measured Toriyama SVG primitives and official color palette tokens in pure Rust.
//!
//! Mirrors `packages/ui/src/toriyama-svg/*` so `shenron-site` (Dioxus 0.8 SSR) and
//! `shenron-admin-ui` render identical vector marks without TypeScript runtime dependencies.

/// Measured Dragon Ball palette constants (tome 1 amber sphere, Goku gi, Kame, Capsule, Shenron).
pub struct ToriyamaPalette;

impl ToriyamaPalette {
    pub const GOKU_ORANGE: &'static str = "#f85b1a";
    pub const GOKU_BLUE: &'static str = "#1b365d";
    pub const GOKU_RED: &'static str = "#de1b22";
    pub const SPHERE_AMBER_LIGHT: &'static str = "#fef48f";
    pub const SPHERE_AMBER_MID: &'static str = "#f9a61a";
    pub const SPHERE_AMBER_DEEP: &'static str = "#d65a0b";
    pub const SPHERE_STAR_RED: &'static str = "#c8231c";
    pub const SPHERE_INK: &'static str = "#18130e";
    pub const KINTO_UN_GOLD: &'static str = "#f5d04c";
    pub const KINTO_UN_SHADE: &'static str = "#d99b1e";
    pub const SENZU_GREEN: &'static str = "#6da34d";
    pub const POTARA_GOLD: &'static str = "#e8b923";
    pub const POTARA_GREEN: &'static str = "#2f8f5b";
    pub const SCOUTER_GREEN: &'static str = "#00ff66";
    pub const SHENRON_EMERALD: &'static str = "#2d6a4f";
}

/// Renders the measured 4-star Dragon Ball sphere (Sūshinchū) as standalone SVG markup.
#[must_use]
pub fn dragon_ball_sphere_svg(size: u32, aria_label: Option<&str>) -> String {
    let a11y = match aria_label {
        Some(label) => format!(r#"role="img" aria-label="{}""#, escape_attr(label)),
        None => r#"aria-hidden="true""#.to_owned(),
    };
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="{size}" height="{size}" {a11y}>
  <defs>
    <radialGradient id="sr-db-amber" cx="35%" cy="30%" r="68%">
      <stop offset="0%" stop-color="{light}"/>
      <stop offset="48%" stop-color="{mid}"/>
      <stop offset="100%" stop-color="{deep}"/>
    </radialGradient>
  </defs>
  <circle cx="50" cy="50" r="46" fill="url(#sr-db-amber)" stroke="{ink}" stroke-width="3.5"/>
  <ellipse cx="34" cy="28" rx="14" ry="8" transform="rotate(-24 34 28)" fill="#ffffff" fill-opacity="0.52"/>
  <g fill="{star}">
    <polygon points="50,24 52.6,31.2 60.2,31.2 54.1,35.6 56.4,42.8 50,38.4 43.6,42.8 45.9,35.6 39.8,31.2 47.4,31.2"/>
    <polygon points="33,44 35.6,51.2 43.2,51.2 37.1,55.6 39.4,62.8 33,58.4 26.6,62.8 28.9,55.6 22.8,51.2 30.4,51.2"/>
    <polygon points="67,44 69.6,51.2 77.2,51.2 71.1,55.6 73.4,62.8 67,58.4 60.6,62.8 62.9,55.6 56.8,51.2 64.4,51.2"/>
    <polygon points="50,59 52.6,66.2 60.2,66.2 54.1,70.6 56.4,77.8 50,73.4 43.6,77.8 45.9,70.6 39.8,66.2 47.4,66.2"/>
  </g>
</svg>"##,
        light = ToriyamaPalette::SPHERE_AMBER_LIGHT,
        mid = ToriyamaPalette::SPHERE_AMBER_MID,
        deep = ToriyamaPalette::SPHERE_AMBER_DEEP,
        ink = ToriyamaPalette::SPHERE_INK,
        star = ToriyamaPalette::SPHERE_STAR_RED,
    )
}

/// Renders the measured Kinto-Un golden cloud mark as standalone SVG markup.
#[must_use]
pub fn kinto_un_svg(width: u32) -> String {
    let height = (width * 56) / 120;
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 56" width="{width}" height="{height}" aria-hidden="true">
  <path d="M18 42 C8 42 4 33 10 26 C8 17 18 10 28 14 C35 5 52 4 61 12 C71 6 88 10 92 21 C104 20 114 29 109 38 C105 44 96 45 86 44 L24 44 C21 44 19 43 18 42 Z"
        fill="{gold}" stroke="{ink}" stroke-width="3" stroke-linejoin="round"/>
  <path d="M24 38 C42 40 76 40 98 36" fill="none" stroke="{shade}" stroke-width="3" stroke-linecap="round"/>
</svg>"##,
        gold = ToriyamaPalette::KINTO_UN_GOLD,
        shade = ToriyamaPalette::KINTO_UN_SHADE,
        ink = ToriyamaPalette::SPHERE_INK,
    )
}

/// Renders the Dragon Ball FR inline wordmark SVG for header and hero lockups.
#[must_use]
pub fn wordmark_svg(height: u32) -> String {
    let width = (height * 220) / 36;
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 220 36" width="{width}" height="{height}" role="img" aria-label="Dragon Ball FR">
  <text x="4" y="27" font-family="Impact, 'Arial Black', sans-serif" font-size="25" font-weight="900" letter-spacing="0.5"
        fill="{orange}" stroke="{ink}" stroke-width="2" paint-order="stroke">DRAGON BALL</text>
  <rect x="176" y="7" width="38" height="22" rx="5" fill="{red}" stroke="{ink}" stroke-width="1.8"/>
  <text x="195" y="23" text-anchor="middle" font-family="system-ui, sans-serif" font-size="14" font-weight="900" fill="#ffffff">FR</text>
</svg>"##,
        orange = ToriyamaPalette::GOKU_ORANGE,
        red = ToriyamaPalette::GOKU_RED,
        ink = ToriyamaPalette::SPHERE_INK,
    )
}

fn escape_attr(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_contains_four_stars_and_measured_amber() {
        let svg = dragon_ball_sphere_svg(48, Some("Sūshinchū"));
        assert!(svg.contains(ToriyamaPalette::SPHERE_AMBER_LIGHT));
        assert!(svg.contains(ToriyamaPalette::SPHERE_STAR_RED));
        assert_eq!(svg.matches("<polygon").count(), 4);
        assert!(svg.contains(r#"aria-label="Sūshinchū""#));
    }

    #[test]
    fn kinto_un_and_wordmark_render_valid_svg() {
        let cloud = kinto_un_svg(120);
        assert!(cloud.starts_with("<svg"));
        assert!(cloud.contains(ToriyamaPalette::KINTO_UN_GOLD));
        let mark = wordmark_svg(36);
        assert!(mark.contains("DRAGON BALL"));
        assert!(mark.contains(">FR<"));
    }
}
