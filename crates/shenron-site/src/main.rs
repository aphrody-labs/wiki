// SPDX-License-Identifier: Apache-2.0
//! Multi-target entrypoint for `shenron-site` (Dioxus 0.8 + Blitz + Axum 0.8):
//! - `--features server`: Axum 0.8.9 HTTP server with `/healthz`, `/health`, upstream API health
//!   probe, optional ISRG incremental static cache (`SHENRON_SITE_ISRG_DIR`), and Dioxus 0.8 SSR
//!   (`DioxusRouterExt::serve_dioxus_application`) bound to `SHENRON_SITE_ADDR` (`127.0.0.1:3010`).
//! - `--features web`: WebAssembly client hydration via `dioxus::launch(App)`.
//! - `--features native`: Blitz GPU (`vello-hybrid`) desktop/mobile native window via
//!   `dioxus::launch(App)`.

use shenron_site::app::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> std::process::ExitCode {
    if let Err(err) = run_server().await {
        tracing::error!(error = %err, "shenron-site server exited with error");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}

#[cfg(feature = "server")]
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{
        Json, Router,
        extract::State,
        http::{StatusCode, header::CACHE_CONTROL},
        response::{IntoResponse, Response},
        routing::get,
    };
    use dioxus::server::{DioxusRouterExt, IncrementalRendererConfig, ServeConfig};
    use serde_json::json;
    use shenron_site::api::{ApiClient, resolve_api_base_url};
    use std::net::SocketAddr;
    use std::sync::Arc;
    use std::time::Duration;

    #[derive(Clone)]
    struct ServerState {
        api_url: Arc<str>,
        api_client: Option<ApiClient>,
    }

    async fn healthz(State(state): State<ServerState>) -> Response {
        (
            [(CACHE_CONTROL, "no-store")],
            Json(json!({
                "status": "ok",
                "service": "shenron-site",
                "version": env!("CARGO_PKG_VERSION"),
                "api_url": state.api_url.as_ref(),
            })),
        )
            .into_response()
    }

    async fn upstream_health(State(state): State<ServerState>) -> Response {
        let Some(client) = state.api_client.as_ref() else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                [(CACHE_CONTROL, "no-store")],
                Json(json!({
                    "status": "degraded",
                    "upstream": state.api_url.as_ref(),
                    "error": "invalid SHENRON_API_URL",
                })),
            )
                .into_response();
        };

        match client.health().await {
            Ok(health) => (
                StatusCode::OK,
                [(CACHE_CONTROL, "no-store")],
                Json(json!({
                    "status": health.status,
                    "upstream": client.base_url(),
                })),
            )
                .into_response(),
            Err(err) => (
                StatusCode::BAD_GATEWAY,
                [(CACHE_CONTROL, "no-store")],
                Json(json!({
                    "status": "unreachable",
                    "upstream": client.base_url(),
                    "error": err.to_string(),
                })),
            )
                .into_response(),
        }
    }

    let _ = rustls::crypto::ring::default_provider().install_default();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("shenron_site=info,warn"));
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();

    let bind_raw = std::env::var("SHENRON_SITE_ADDR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "127.0.0.1:3010".to_owned());
    let addr: SocketAddr =
        bind_raw.parse().map_err(|err| format!("invalid SHENRON_SITE_ADDR `{bind_raw}`: {err}"))?;

    let api_url = resolve_api_base_url();
    let api_client = match ApiClient::new(&api_url) {
        Ok(client) => Some(client),
        Err(err) => {
            tracing::warn!(%api_url, error = %err, "SHENRON_API_URL could not be initialized");
            None
        },
    };

    let state = ServerState { api_url: Arc::from(api_url), api_client };

    let mut serve_cfg = ServeConfig::new();
    if let Ok(cache_dir) = std::env::var("SHENRON_SITE_ISRG_DIR")
        && !cache_dir.trim().is_empty()
    {
        let ttl_secs = std::env::var("SHENRON_SITE_ISRG_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60);
        serve_cfg = serve_cfg.incremental(
            IncrementalRendererConfig::new()
                .static_dir(cache_dir.trim())
                .invalidate_after(Duration::from_secs(ttl_secs)),
        );
    }

    async fn robots_txt() -> Response {
        (
            [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            "User-agent: *\nAllow: /\nDisallow: /admin\nSitemap: https://dragonballfr.com/sitemap.xml\n",
        )
            .into_response()
    }

    async fn sitemap_xml() -> Response {
        let urls = [
            "https://dragonballfr.com/",
            "https://tv.dragonballfr.com/",
            "https://tv.dragonballfr.com/films",
            "https://wiki.dragonballfr.com/",
            "https://wiki.dragonballfr.com/personnages",
            "https://wiki.dragonballfr.com/sagas",
            "https://wiki.dragonballfr.com/episodes",
            "https://wiki.dragonballfr.com/films",
            "https://wiki.dragonballfr.com/chronologie",
            "https://wiki.dragonballfr.com/sources",
            "https://manga.dragonballfr.com/",
            "https://games.dragonballfr.com/",
        ];
        let mut body = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        );
        for url in urls {
            body.push_str("<url><loc>");
            body.push_str(url);
            body.push_str("</loc></url>");
        }
        body.push_str("</urlset>");
        (
            [
                (axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8"),
                (CACHE_CONTROL, "public, max-age=3600"),
            ],
            body,
        )
            .into_response()
    }

    let api_router = Router::new()
        .route("/healthz", get(healthz))
        .route("/health", get(healthz))
        .route("/robots.txt", get(robots_txt))
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/api/site/upstream-health", get(upstream_health))
        .with_state(state);

    let dioxus_router = Router::new().serve_dioxus_application(serve_cfg, App);
    let router = api_router.merge(dioxus_router);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "shenron-site listening");
    axum::serve(listener, router).with_graceful_shutdown(shutdown_signal()).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut term) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = term.recv() => {}
                }
            },
            Err(err) => {
                tracing::warn!(error = %err, "failed to install SIGTERM handler");
                let _ = tokio::signal::ctrl_c().await;
            },
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
