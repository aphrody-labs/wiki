// SPDX-License-Identifier: Apache-2.0
//! Public Dragon Ball FR portal on Dioxus 0.8 + Blitz GPU + Axum 0.8 SSR (ADR 0010).
//!
//! Architecture boundaries:
//! - UI components never connect directly to PostgreSQL or Redis; every read flows over HTTP
//!   through [`api::ApiClient`] targeting `SHENRON_API_URL` (and `SHENRON_TV_URL` for TV items).
//! - All visual tokens live in [`theme`] and mirror the WCAG AA/AAA 3-mode tokens verified in
//!   `replica/design/tokens.css` and `replica/design/themes-3modes.css`.

pub mod api;
pub mod app;
pub mod icons;
pub mod routes;
pub mod theme;
