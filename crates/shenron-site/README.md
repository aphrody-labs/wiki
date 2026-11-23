<!-- SPDX-License-Identifier: Apache-2.0 -->
# `shenron-site`

Portail Dragon Ball FR multi-cible construit en Rust sur notre pile **Dioxus 0.8 (`0.8.0-alpha.3`) + Blitz (`2026.11.0`, `vello-hybrid`) + Axum `0.8.9` SSR** (ADR 0010).

## Principes d’architecture

- **Crate autonome sur le checkout frère `aphrody`** : exclu du workspace racine `wiki/Cargo.toml` (comme `shenron-deployd` et `shenron-tv`), pointant vers `../../../aphrody/src/dioxus` avec les trois cibles natives :
  - `--features server` : serveur SSR Axum `0.8.9` + `DioxusRouterExt` + ISRG optionnel (`SHENRON_SITE_ISRG_DIR`) + `reqwest 0.13.5` / `rustls 0.23.46` (`ring`).
  - `--features web` : hydratation navigateur WebAssembly.
  - `--features native` : rendu GPU natif `dioxus-native 0.8.0-alpha.3` + `blitz 2026.11.0` (`vello-hybrid`).
- **Coquille vivante à 4 piliers pilotée par les frames CDN** :
  1. **Théâtre Anime** (`/tv`, `/:lang/tv`) : frames animées en temps réel et catalogue HLS `shenron-tv`.
  2. **Lecteur Manga** (`/wiki/manga`, `/:lang/wiki/manga`) : planches CDN haute définition en simple ou double page.
  3. **Jeux natifs WebGPU / WASM** (`/games`, `/:lang/games`) : lanceur Budokai Tenkaichi 3, Xenoverse 2 et Dragon Pixel.
  4. **Encyclopédie et chronologie** (`/wiki/episodes`, `/wiki/episodes/:slug`, `/wiki/movies`, `/wiki/timeline`).
- **Zéro accès direct aux bases de données** : `src/api.rs` consomme uniquement l’API HTTP publique (`${SHENRON_API_URL}/api/v1/frames/live`, `/api/public/wiki/*`, `/api/v1/wiki/timeline/events`) et `SHENRON_TV_URL`.
- **Jetons visuels WCAG AA/AAA et craft-floor** : `src/theme.rs` embarque les trois modes vérifiés (`Neutre`, `Goku SSG`, `Beerus`) alignés sur `replica/design/tokens.css` et `replica/design/themes-3modes.css`.

## Variables d’environnement

| Variable | Valeur par défaut | Rôle |
| --- | --- | --- |
| `SHENRON_SITE_ADDR` | `127.0.0.1:3010` | Adresse d’écoute du serveur Axum SSR |
| `SHENRON_API_URL` | `http://127.0.0.1:3000` | Origine HTTP de l’API publique Shenron (`/api/v1/frames/live`, `/api/public/wiki/*`) |
| `SHENRON_TV_URL` | `http://127.0.0.1:8793` | Origine HTTP du catalogue `shenron-tv` |
| `SHENRON_SITE_ISRG_DIR` | *(désactivé)* | Répertoire de cache incrémental ISRG (`IncrementalRendererConfig`) |
| `SHENRON_SITE_ISRG_TTL_SECS` | `60` | Durée de validité ISRG en secondes |
| `RUST_LOG` | `shenron_site=info,warn` | Niveau de journalisation `tracing` |

## Développement local

```bash
cd crates/shenron-site
dx serve --platform web
```

## Compilation release (sur `fr1` uniquement)

Les compilations Rust lourdes s’exécutent exclusivement sur l’hôte de build `fr1`, jamais sur le poste Windows :

```bash
timeout 180 ssh fr1 'cd ~/wiki/crates/shenron-site && cargo build --release --features server'
```

## Points de contrôle HTTP

- `GET /healthz` (alias `/health`) : renvoie `{ "status": "ok", "service": "shenron-site", "version": "...", "api_url": "..." }` avec `Cache-Control: no-store`.
- `GET /api/site/upstream-health` : sonde `GET /api/health` sur `SHENRON_API_URL` et renvoie `200 OK` ou `502 Bad Gateway`.
