// SPDX-License-Identifier: Apache-2.0
//! Dioxus 0.7 application root, accessible HTML5 layout shell, and 4-pillar live CDN frame views.
//!
//! Every view follows the Shenron craft floor:
//! - Real-time CDN frames fetched from `${SHENRON_API_URL}/api/v1/frames/live` drive the stage,
//!   Anime Theater, Manga Spread Reader, Native Games launcher, and Wiki Encyclopedia.
//! - Zero eyebrow kickers above headings, zero gradient text, zero emoji, zero nested cards.
//! - Semantic HTML5 landmarks (`header`, `nav`, `main`, `article`, `section`, `footer`),
//!   WCAG AA/AAA 3-mode themes (`Neutre`, `Goku SSG`, `Beerus`), and >=44px interactive controls.

use dioxus::prelude::*;

use crate::api::{
    EpisodeRecord, LiveFrame, MovieRecord, TimelineEvent, TvVideo, load_episode_detail,
    load_episodes, load_live_frames, load_movies, load_timeline_events, load_tv_videos,
};
use crate::icons::{
    ICON_ADMIN, ICON_DISCORD, ICON_GAMES, ICON_GITHUB, ICON_GLOBE, ICON_HOME, ICON_MANGA,
    ICON_SEARCH, ICON_THEME_BEERUS, ICON_THEME_GOKU, ICON_THEME_NEUTRAL, ICON_TV, ICON_WIKI,
};
use crate::routes::{Locale, Route};
use crate::theme::{SITE_STYLESHEET, ThemeMode, toriyama_svg};

/// Localized static UI copy for all 4 living pillars.
#[derive(Debug, Clone, Copy)]
struct Copy {
    skip_to_content: &'static str,
    nav_label: &'static str,
    nav_home: &'static str,
    nav_tv: &'static str,
    nav_manga: &'static str,
    nav_games: &'static str,
    nav_episodes: &'static str,
    nav_movies: &'static str,
    nav_timeline: &'static str,
    theme_group_label: &'static str,
    locale_group_label: &'static str,
    hero_title: &'static str,
    hero_lead: &'static str,
    hero_cta_tv: &'static str,
    hero_cta_episodes: &'static str,
    frame_strip_label: &'static str,
    pillars_heading: &'static str,
    tv_title: &'static str,
    tv_lead: &'static str,
    manga_title: &'static str,
    manga_lead: &'static str,
    manga_spread_btn: &'static str,
    manga_single_btn: &'static str,
    games_title: &'static str,
    games_lead: &'static str,
    episodes_title: &'static str,
    episodes_lead: &'static str,
    episodes_search_label: &'static str,
    episodes_search_placeholder: &'static str,
    movies_title: &'static str,
    movies_lead: &'static str,
    timeline_title: &'static str,
    timeline_lead: &'static str,
    timeline_all_eras: &'static str,
    loading: &'static str,
    empty: &'static str,
    api_unavailable: &'static str,
    back_to_episodes: &'static str,
    not_found_title: &'static str,
    not_found_body: &'static str,
    footer_note: &'static str,
}

const fn copy_for(locale: Locale) -> Copy {
    match locale {
        Locale::Fr => Copy {
            skip_to_content: "Aller au contenu principal",
            nav_label: "Navigation principale",
            nav_home: "Accueil",
            nav_tv: "Anime",
            nav_manga: "Manga",
            nav_games: "Jeux natifs",
            nav_episodes: "Épisodes",
            nav_movies: "Films",
            nav_timeline: "Chronologie",
            theme_group_label: "Choix du thème visuel",
            locale_group_label: "Choix de la langue",
            hero_title: "Dragon Ball FR — Anime, manga, jeux natifs et encyclopédie",
            hero_lead: "Chaque scène est extraite en temps réel depuis le CDN Shenron pour composer l’interface web, bureau et mobile sur Rust et Dioxus SSR.",
            hero_cta_tv: "Ouvrir le théâtre anime",
            hero_cta_episodes: "Explorer l’encyclopédie",
            frame_strip_label: "Sélectionner une frame CDN en direct",
            pillars_heading: "Les quatre piliers vivants",
            tv_title: "Théâtre Anime — Shenron TV",
            tv_lead: "Flux HLS et frames animées en direct servis par l’API publique et Shenron TV.",
            manga_title: "Lecteur Manga — Planches CDN",
            manga_lead: "Planches haute définition servies en temps réel pour la lecture simple ou double page.",
            manga_spread_btn: "Double page",
            manga_single_btn: "Page simple",
            games_title: "Jeux natifs WebGPU et WASM",
            games_lead: "Lanceur natif pour Budokai Tenkaichi 3, Xenoverse 2 et Dragon Pixel, illustré par les captures temps réel du CDN.",
            episodes_title: "Guide des épisodes",
            episodes_lead: "Catalogue complet des épisodes Dragon Ball, Z, GT, Super et Daima via l’API HTTP.",
            episodes_search_label: "Filtrer les épisodes",
            episodes_search_placeholder: "Titre, série ou numéro d’épisode…",
            movies_title: "Films et téléfilms",
            movies_lead: "Longs-métrages Dragon Ball indexés par l’API publique Shenron.",
            timeline_title: "Chronologie de l’univers",
            timeline_lead: "Repères canoniques et dates clés classés par ère.",
            timeline_all_eras: "Toutes les ères",
            loading: "Chargement depuis l’API Shenron…",
            empty: "Aucune donnée disponible pour cette sélection.",
            api_unavailable: "Impossible de joindre l’API Shenron : vérifiez SHENRON_API_URL et réessayez.",
            back_to_episodes: "Retour au guide des épisodes",
            not_found_title: "Page introuvable",
            not_found_body: "L’adresse demandée ne correspond à aucune page du portail.",
            footer_note: "Dragon Ball FR — Dioxus 0.8 SSR, Blitz GPU et Axum 0.8.",
        },
        Locale::En => Copy {
            skip_to_content: "Skip to main content",
            nav_label: "Primary navigation",
            nav_home: "Home",
            nav_tv: "Anime",
            nav_manga: "Manga",
            nav_games: "Native Games",
            nav_episodes: "Episodes",
            nav_movies: "Movies",
            nav_timeline: "Timeline",
            theme_group_label: "Visual theme switcher",
            locale_group_label: "Language switcher",
            hero_title: "Dragon Ball FR — Anime, manga, native games, and encyclopedia",
            hero_lead: "Real-time frames from the Shenron CDN drive the living web, desktop, and mobile shell on Rust and Dioxus SSR.",
            hero_cta_tv: "Open Anime Theater",
            hero_cta_episodes: "Browse encyclopedia",
            frame_strip_label: "Select a live CDN frame",
            pillars_heading: "Four living pillars",
            tv_title: "Anime Theater — Shenron TV",
            tv_lead: "Live HLS streams and CDN frames served through the public Shenron HTTP API.",
            manga_title: "Manga Spread Reader",
            manga_lead: "Live CDN manga spreads with single-page and double-page reading modes.",
            manga_spread_btn: "Double spread",
            manga_single_btn: "Single page",
            games_title: "Native WebGPU & WASM Games",
            games_lead: "Native launcher for Budokai Tenkaichi 3, Xenoverse 2, and Dragon Pixel powered by live CDN frames.",
            episodes_title: "Episode Guide",
            episodes_lead: "Complete episode catalog across Dragon Ball, Z, GT, Super, and Daima via the HTTP API.",
            episodes_search_label: "Filter episodes",
            episodes_search_placeholder: "Search by title, series, or episode number…",
            movies_title: "Movies & Specials",
            movies_lead: "Feature films indexed through the public Shenron API.",
            timeline_title: "Universe Timeline",
            timeline_lead: "Canonical milestones and key dates organized by era.",
            timeline_all_eras: "All eras",
            loading: "Loading from the Shenron API…",
            empty: "No entries available for this selection.",
            api_unavailable: "Unable to reach the Shenron API: check SHENRON_API_URL and try again.",
            back_to_episodes: "Back to episode guide",
            not_found_title: "Page not found",
            not_found_body: "The requested address does not match any page on this portal.",
            footer_note: "Dragon Ball FR — Dioxus 0.8 SSR, Blitz GPU, and Axum 0.8.",
        },
        Locale::Ja => Copy {
            skip_to_content: "メインコンテンツへ移動",
            nav_label: "メインナビゲーション",
            nav_home: "ホーム",
            nav_tv: "アニメ",
            nav_manga: "マンガ",
            nav_games: "ネイティブゲーム",
            nav_episodes: "エピソード",
            nav_movies: "劇場版",
            nav_timeline: "年表",
            theme_group_label: "テーマ切替",
            locale_group_label: "言語切替",
            hero_title: "Dragon Ball FR — アニメ・マンガ・ゲーム・百科事典",
            hero_lead: "Shenron CDN からリアルタイムに取得したフレームで、Web・デスクトップ・モバイル UI を駆動します。",
            hero_cta_tv: "アニメシアターを開く",
            hero_cta_episodes: "百科事典を見る",
            frame_strip_label: "ライブ CDN フレームを選択",
            pillars_heading: "4 つのライブ領域",
            tv_title: "アニメシアター — Shenron TV",
            tv_lead: "公開 API と Shenron TV から配信される HLS 映像とリアルタイムフレームです。",
            manga_title: "マンガ見開きリーダー",
            manga_lead: "CDN から配信される高精細ページを見開きまたは単ページで閲覧できます。",
            manga_spread_btn: "見開き",
            manga_single_btn: "単ページ",
            games_title: "WebGPU / WASM ネイティブゲーム",
            games_lead: "Budokai Tenkaichi 3・Xenoverse 2・Dragon Pixel のランチャー。",
            episodes_title: "エピソード一覧",
            episodes_lead: "ドラゴンボール各シリーズのエピソード情報。",
            episodes_search_label: "エピソード検索",
            episodes_search_placeholder: "タイトル・シリーズ・話数で絞り込み…",
            movies_title: "劇場版・スペシャル",
            movies_lead: "公開 API から取得した映画一覧です。",
            timeline_title: "ドラゴンボール年表",
            timeline_lead: "宇宙の歴史と主要な出来事を時代別に表示します。",
            timeline_all_eras: "すべての時代",
            loading: "Shenron API から読み込み中…",
            empty: "該当するデータがありません。",
            api_unavailable: "Shenron API に接続できません。SHENRON_API_URL を確認してください。",
            back_to_episodes: "エピソード一覧に戻る",
            not_found_title: "ページが見つかりません",
            not_found_body: "指定された URL は存在しません。",
            footer_note: "Dragon Ball FR — Dioxus 0.8 SSR + Blitz GPU + Axum 0.8.",
        },
    }
}

/// Root fullstack Dioxus application component.
#[component]
pub fn App() -> Element {
    let theme = use_context_provider(|| Signal::new(ThemeMode::Neutral));
    let locale = use_context_provider(|| Signal::new(Locale::Fr));

    let active_theme = *theme.read();
    let active_locale = *locale.read();

    rsx! {
        document::Title { "Shenron — Dragon Ball FR" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { name: "color-scheme", content: "dark" }
        document::Meta { name: "theme-color", content: active_theme.meta_color() }
        document::Style { "{SITE_STYLESHEET}" }

        div {
            class: "sr-root",
            "data-shenron-theme": active_theme.as_attr(),
            lang: active_locale.bcp47(),
            Router::<Route> {}
        }
    }
}

/// Semantic HTML5 layout shell shared by every portal route.
#[component]
pub fn SiteShell() -> Element {
    let mut theme = use_context::<Signal<ThemeMode>>();
    let mut locale = use_context::<Signal<Locale>>();
    let route = use_route::<Route>();
    let navigator = use_navigator();

    let effective_locale = route.locale().unwrap_or_else(|| *locale.read());
    use_effect(move || {
        if *locale.read() != effective_locale {
            locale.set(effective_locale);
        }
    });

    let copy = copy_for(effective_locale);
    let active_theme = *theme.read();
    let lang_code = effective_locale.code().to_owned();

    let is_home = matches!(route, Route::Home {} | Route::LocaleHome { .. });
    let is_tv = matches!(route, Route::Tv {} | Route::LocaleTv { .. });
    let is_manga = matches!(
        route,
        Route::Manga {}
            | Route::LocaleManga { .. }
            | Route::MangaStudio {}
            | Route::LocaleMangaStudio { .. }
    );
    let is_games = matches!(route, Route::Games {} | Route::LocaleGames { .. });
    let is_episodes = matches!(
        route,
        Route::Episodes {}
            | Route::LocaleEpisodes { .. }
            | Route::EpisodeDetail { .. }
            | Route::LocaleEpisodeDetail { .. }
    );
    let is_movies = matches!(route, Route::Movies {} | Route::LocaleMovies { .. });
    let is_timeline = matches!(route, Route::Timeline {} | Route::LocaleTimeline { .. });
    let is_wiki = is_episodes
        || is_movies
        || is_timeline
        || matches!(
            route,
            Route::WikiHub {}
                | Route::LocaleWikiHub { .. }
                | Route::Characters {}
                | Route::LocaleCharacters { .. }
                | Route::Sagas {}
                | Route::LocaleSagas { .. }
                | Route::Sources {}
                | Route::LocaleSources { .. }
                | Route::Databooks {}
                | Route::LocaleDatabooks { .. }
        );
    let is_search = matches!(route, Route::Search {} | Route::LocaleSearch { .. });
    let is_admin = matches!(route, Route::Admin {} | Route::LocaleAdmin { .. });

    let wordmark_html = toriyama_svg::wordmark_svg(32);
    let sphere_html = toriyama_svg::dragon_ball_sphere_svg(28, Some("Dragon Ball FR"));

    rsx! {
        a { class: "sr-skip-link", href: "#main-content", "{copy.skip_to_content}" }

        aside { class: "sr-app-rail", aria_label: copy.nav_label,
            Link {
                class: "sr-rail-brand",
                to: Route::LocaleHome { lang: lang_code.clone() },
                aria_label: "Dragon Ball FR",
                span { dangerous_inner_html: "{sphere_html}" }
            }
            nav { class: "sr-rail-nav", aria_label: copy.nav_label,
                Link {
                    class: "sr-rail-item",
                    to: Route::LocaleHome { lang: lang_code.clone() },
                    aria_current: if is_home { Some("page") } else { None },
                    title: copy.nav_home,
                    span { class: "sr-rail-icon", dangerous_inner_html: "{ICON_HOME}" }
                    span { class: "sr-rail-label", "{copy.nav_home}" }
                }
                Link {
                    class: "sr-rail-item",
                    to: Route::Tv {},
                    aria_current: if is_tv { Some("page") } else { None },
                    title: copy.nav_tv,
                    span { class: "sr-rail-icon", dangerous_inner_html: "{ICON_TV}" }
                    span { class: "sr-rail-label", "{copy.nav_tv}" }
                }
                Link {
                    class: "sr-rail-item",
                    to: Route::WikiHub {},
                    aria_current: if is_wiki { Some("page") } else { None },
                    title: "Wiki",
                    span { class: "sr-rail-icon", dangerous_inner_html: "{ICON_WIKI}" }
                    span { class: "sr-rail-label", "Wiki" }
                }
                Link {
                    class: "sr-rail-item",
                    to: Route::MangaStudio {},
                    aria_current: if is_manga { Some("page") } else { None },
                    title: copy.nav_manga,
                    span { class: "sr-rail-icon", dangerous_inner_html: "{ICON_MANGA}" }
                    span { class: "sr-rail-label", "{copy.nav_manga}" }
                }
                Link {
                    class: "sr-rail-item",
                    to: Route::Games {},
                    aria_current: if is_games { Some("page") } else { None },
                    title: copy.nav_games,
                    span { class: "sr-rail-icon", dangerous_inner_html: "{ICON_GAMES}" }
                    span { class: "sr-rail-label", "{copy.nav_games}" }
                }
            }
            div { class: "sr-rail-footer",
                Link {
                    class: "sr-rail-icon-btn",
                    to: Route::Admin {},
                    aria_current: if is_admin { Some("page") } else { None },
                    aria_label: "Administration",
                    title: "Administration",
                    span { dangerous_inner_html: "{ICON_ADMIN}" }
                }
            }
        }

        header { class: "sr-header", role: "banner",
            div { class: "sr-header-inner",
                Link {
                    class: "sr-brand",
                    to: Route::LocaleHome { lang: lang_code.clone() },
                    aria_label: "Dragon Ball FR",
                    span { dangerous_inner_html: "{wordmark_html}" }
                }

                nav { class: "sr-nav", aria_label: copy.nav_label,
                    Link {
                        class: "sr-nav-link",
                        to: Route::WikiHub {},
                        aria_current: if is_wiki { Some("page") } else { None },
                        span { class: "sr-inline-icon", dangerous_inner_html: "{ICON_WIKI}" }
                        span { "Wiki" }
                    }
                    Link {
                        class: "sr-nav-link",
                        to: Route::Tv {},
                        aria_current: if is_tv { Some("page") } else { None },
                        span { class: "sr-inline-icon", dangerous_inner_html: "{ICON_TV}" }
                        span { "{copy.nav_tv}" }
                    }
                    Link {
                        class: "sr-nav-link",
                        to: Route::MangaStudio {},
                        aria_current: if is_manga { Some("page") } else { None },
                        span { class: "sr-inline-icon", dangerous_inner_html: "{ICON_MANGA}" }
                        span { "{copy.nav_manga}" }
                    }
                    Link {
                        class: "sr-nav-link",
                        to: Route::Games {},
                        aria_current: if is_games { Some("page") } else { None },
                        span { class: "sr-inline-icon", dangerous_inner_html: "{ICON_GAMES}" }
                        span { "{copy.nav_games}" }
                    }
                    Link {
                        class: "sr-nav-link",
                        to: Route::Timeline {},
                        aria_current: if is_timeline { Some("page") } else { None },
                        span { "{copy.nav_timeline}" }
                    }
                }

                div { class: "sr-controls",
                    Link {
                        class: "sr-icon-link",
                        to: Route::Search {},
                        aria_current: if is_search { Some("page") } else { None },
                        aria_label: "Rechercher",
                        title: "Rechercher",
                        span { dangerous_inner_html: "{ICON_SEARCH}" }
                    }
                    a {
                        class: "sr-icon-link",
                        href: "https://discord.gg/dragonballfr",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        aria_label: "Discord Dragon Ball FR",
                        title: "Discord",
                        span { dangerous_inner_html: "{ICON_DISCORD}" }
                    }
                    a {
                        class: "sr-icon-link",
                        href: "https://github.com/aphrody-labs/shenron",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        aria_label: "GitHub Aphrody Labs",
                        title: "GitHub",
                        span { dangerous_inner_html: "{ICON_GITHUB}" }
                    }

                    div {
                        class: "sr-control-group",
                        role: "radiogroup",
                        aria_label: copy.theme_group_label,
                        for mode in ThemeMode::ALL {
                            {
                                let icon = match mode {
                                    ThemeMode::Neutral => ICON_THEME_NEUTRAL,
                                    ThemeMode::Goku => ICON_THEME_GOKU,
                                    ThemeMode::Beerus => ICON_THEME_BEERUS,
                                };
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: "sr-chip-btn",
                                        role: "radio",
                                        aria_checked: if active_theme == mode { "true" } else { "false" },
                                        aria_label: mode.label(),
                                        title: mode.label(),
                                        onclick: move |_| theme.set(mode),
                                        span { class: "sr-inline-icon", dangerous_inner_html: "{icon}" }
                                        span { "{mode.label()}" }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "sr-control-group",
                        role: "radiogroup",
                        aria_label: copy.locale_group_label,
                        span { class: "sr-inline-icon sr-locale-globe", dangerous_inner_html: "{ICON_GLOBE}" }
                        for candidate in Locale::ALL {
                            {
                                let target_route = route.with_locale(candidate);
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: "sr-chip-btn",
                                        role: "radio",
                                        aria_checked: if effective_locale == candidate { "true" } else { "false" },
                                        onclick: move |_| {
                                            locale.set(candidate);
                                            navigator.push(target_route.clone());
                                        },
                                        "{candidate.short_label()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        main { id: "main-content", class: "sr-main", tabindex: "-1",
            Outlet::<Route> {}
        }

        footer { class: "sr-footer", role: "contentinfo",
            div { class: "sr-footer-inner",
                span { "{copy.footer_note}" }
                span { class: "sr-tabular", "{effective_locale.bcp47()} · {active_theme.label()}" }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! { HomeView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleHome(lang: String) -> Element {
    rsx! { HomeView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Tv() -> Element {
    rsx! { TvView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleTv(lang: String) -> Element {
    rsx! { TvView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Manga() -> Element {
    rsx! { MangaView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleManga(lang: String) -> Element {
    rsx! { MangaView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Games() -> Element {
    rsx! { GamesView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleGames(lang: String) -> Element {
    rsx! { GamesView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Episodes() -> Element {
    rsx! { EpisodesView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleEpisodes(lang: String) -> Element {
    rsx! { EpisodesView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn EpisodeDetail(slug: String) -> Element {
    rsx! { EpisodeDetailView { locale: Locale::Fr, slug } }
}

#[component]
pub fn LocaleEpisodeDetail(lang: String, slug: String) -> Element {
    rsx! { EpisodeDetailView { locale: Locale::parse_or_default(&lang), slug } }
}

#[component]
pub fn Movies() -> Element {
    rsx! { MoviesView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleMovies(lang: String) -> Element {
    rsx! { MoviesView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Timeline() -> Element {
    rsx! { TimelineView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleTimeline(lang: String) -> Element {
    rsx! { TimelineView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn MangaStudio() -> Element {
    rsx! { MangaView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleMangaStudio(lang: String) -> Element {
    rsx! { MangaView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn WikiHub() -> Element {
    rsx! { WikiHubView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleWikiHub(lang: String) -> Element {
    rsx! { WikiHubView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Characters() -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::Fr,
            title: "Personnages Dragon Ball",
            lead: "Fiches encyclopédiques canoniques (Dragon Ball, Z, GT, Super, Daima) sourcées depuis les Daizenshuu et Chozenshuu.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/characters",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn LocaleCharacters(lang: String) -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::parse_or_default(&lang),
            title: "Personnages Dragon Ball",
            lead: "Fiches encyclopédiques canoniques (Dragon Ball, Z, GT, Super, Daima) sourcées depuis les Daizenshuu et Chozenshuu.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/characters",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn Sagas() -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::Fr,
            title: "Sagas et arcs narratifs",
            lead: "Les 41 arcs majeurs de l’univers Dragon Ball, du Pilaf Arc jusqu’à Dragon Ball Daima.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/sagas",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn LocaleSagas(lang: String) -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::parse_or_default(&lang),
            title: "Sagas et arcs narratifs",
            lead: "Les 41 arcs majeurs de l’univers Dragon Ball, du Pilaf Arc jusqu’à Dragon Ball Daima.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/sagas",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn Sources() -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::Fr,
            title: "Sources et bibliographie",
            lead: "Références vérifiées : manga Kanzenban, interviews d’Akira Toriyama, Daizenshuu et guides officiels Shueisha.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/sources",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn LocaleSources(lang: String) -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::parse_or_default(&lang),
            title: "Sources et bibliographie",
            lead: "Références vérifiées : manga Kanzenban, interviews d’Akira Toriyama, Daizenshuu et guides officiels Shueisha.",
            api_path: "https://api.dragonballfr.com/api/public/wiki/sources",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn Databooks() -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::Fr,
            title: "Databooks et archives",
            lead: "483 éditions et encyclopédies officielles indexées : Daizenshuu, Chozenshuu, Super Exciting Guides et Landmark.",
            api_path: "https://api.dragonballfr.com/api/v1/databooks",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn LocaleDatabooks(lang: String) -> Element {
    rsx! {
        CatalogPortalView {
            locale: Locale::parse_or_default(&lang),
            title: "Databooks et archives",
            lead: "483 éditions et encyclopédies officielles indexées : Daizenshuu, Chozenshuu, Super Exciting Guides et Landmark.",
            api_path: "https://api.dragonballfr.com/api/v1/databooks",
            back_to: Route::WikiHub {},
        }
    }
}

#[component]
pub fn Search() -> Element {
    rsx! { SearchView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleSearch(lang: String) -> Element {
    rsx! { SearchView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn Admin() -> Element {
    rsx! { AdminView { locale: Locale::Fr } }
}

#[component]
pub fn LocaleAdmin(lang: String) -> Element {
    rsx! { AdminView { locale: Locale::parse_or_default(&lang) } }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let locale = *use_context::<Signal<Locale>>().read();
    let copy = copy_for(locale);
    let missing = segments.join("/");

    rsx! {
        document::Title { "{copy.not_found_title} — Shenron" }
        document::Meta { name: "description", content: copy.not_found_body }

        article { class: "sr-stage-copy",
            h1 { class: "sr-title", "{copy.not_found_title}" }
            p { class: "sr-lead", "{copy.not_found_body} (/{missing})" }
            div { class: "sr-actions",
                Link { class: "sr-cta", to: Route::Home {}, "{copy.nav_home}" }
            }
        }
    }
}

fn find_pillar_frame<'a>(frames: &'a [LiveFrame], pillar: &str) -> Option<&'a LiveFrame> {
    frames.iter().find(|f| f.pillar.eq_ignore_ascii_case(pillar)).or_else(|| frames.first())
}

#[component]
fn HomeView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut active_idx = use_signal(|| 0usize);
    let live = use_server_future(move || load_live_frames(None, 16))?;

    let frames: Vec<LiveFrame> = match &*live.read() {
        Some(Ok(payload)) => payload.frames.clone(),
        _ => Vec::new(),
    };

    let selected = frames.get(*active_idx.read()).or_else(|| frames.first());
    let stage_style = selected
        .and_then(LiveFrame::safe_aura_hex)
        .map(|hex| {
            format!("box-shadow: 0 24px 64px color-mix(in srgb, {hex} 28%, rgba(0, 0, 0, 0.72));")
        })
        .unwrap_or_default();

    let anime_frame = find_pillar_frame(&frames, "anime");
    let manga_frame = find_pillar_frame(&frames, "manga");
    let games_frame = find_pillar_frame(&frames, "games");
    let wiki_frame = find_pillar_frame(&frames, "wiki");

    rsx! {
        document::Title { "Shenron — Dragon Ball FR" }
        document::Meta { name: "description", content: copy.hero_lead }

        section { class: "sr-stage", style: "{stage_style}", aria_labelledby: "home-hero-title",
            div { class: "sr-stage-media",
                if let Some(frame) = selected {
                    img {
                        class: "sr-stage-img",
                        src: "{frame.url}",
                        alt: "{frame.alt_text()}",
                        loading: "eager",
                    }
                    div { class: "sr-stage-scrim" }
                    div { class: "sr-stage-caption",
                        span { "{frame.title}" }
                        if let Some(series) = frame.series.as_deref() {
                            span { class: "sr-tabular", "{series}" }
                        }
                    }
                }
            }

            div { class: "sr-stage-copy",
                h1 { id: "home-hero-title", class: "sr-title", "{copy.hero_title}" }
                p { class: "sr-lead", "{copy.hero_lead}" }
                div { class: "sr-actions",
                    Link { class: "sr-cta", to: Route::Tv {}, "{copy.hero_cta_tv}" }
                    Link { class: "sr-cta-secondary", to: Route::Episodes {}, "{copy.hero_cta_episodes}" }
                }
            }
        }

        if !frames.is_empty() {
            div { class: "sr-frame-strip", role: "group", aria_label: copy.frame_strip_label,
                for (idx, frame) in frames.iter().enumerate() {
                    button {
                        r#type: "button",
                        class: "sr-frame-thumb",
                        aria_label: "{frame.alt_text()}",
                        aria_pressed: if *active_idx.read() == idx { "true" } else { "false" },
                        onclick: move |_| active_idx.set(idx),
                        img {
                            src: "{frame.url}",
                            alt: "{frame.alt_text()}",
                            loading: "lazy",
                        }
                    }
                }
            }
        }

        h2 { class: "sr-section-title", "{copy.pillars_heading}" }
        section { class: "sr-pillars", aria_label: copy.pillars_heading,
            PillarPanel {
                title: copy.tv_title,
                description: copy.tv_lead,
                to: Route::Tv {},
                frame: anime_frame.cloned(),
            }
            PillarPanel {
                title: copy.manga_title,
                description: copy.manga_lead,
                to: Route::Manga {},
                frame: manga_frame.cloned(),
            }
            PillarPanel {
                title: copy.games_title,
                description: copy.games_lead,
                to: Route::Games {},
                frame: games_frame.cloned(),
            }
            PillarPanel {
                title: copy.episodes_title,
                description: copy.episodes_lead,
                to: Route::Episodes {},
                frame: wiki_frame.cloned(),
            }
        }
    }
}

#[component]
fn PillarPanel(
    title: &'static str,
    description: &'static str,
    to: Route,
    frame: Option<LiveFrame>,
) -> Element {
    rsx! {
        article { class: "sr-pillar-panel",
            if let Some(f) = frame {
                img {
                    class: "sr-pillar-media",
                    src: "{f.url}",
                    alt: "{f.alt_text()}",
                    loading: "lazy",
                }
            }
            div { class: "sr-pillar-body",
                h3 { class: "sr-pillar-heading",
                    Link { to, "{title}" }
                }
                p { class: "sr-muted", "{description}" }
            }
        }
    }
}

#[component]
fn TvView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut active_frame = use_signal(|| 0usize);
    let frames = use_server_future(move || load_live_frames(Some("anime".to_owned()), 12))?;
    let videos = use_server_future(load_tv_videos)?;

    let live_frames: Vec<LiveFrame> = match &*frames.read() {
        Some(Ok(payload)) => payload.frames.clone(),
        _ => Vec::new(),
    };
    let current = live_frames.get(*active_frame.read()).or_else(|| live_frames.first());

    rsx! {
        document::Title { "{copy.tv_title} — Shenron" }
        document::Meta { name: "description", content: copy.tv_lead }

        section { class: "sr-stage", aria_labelledby: "tv-title",
            div { class: "sr-stage-media",
                if let Some(frame) = current {
                    img {
                        class: "sr-stage-img",
                        src: "{frame.url}",
                        alt: "{frame.alt_text()}",
                    }
                    div { class: "sr-stage-scrim" }
                    div { class: "sr-stage-caption",
                        span { "{frame.title}" }
                    }
                }
            }
            div { class: "sr-stage-copy",
                h1 { id: "tv-title", class: "sr-title", "{copy.tv_title}" }
                p { class: "sr-lead", "{copy.tv_lead}" }
            }
        }

        if !live_frames.is_empty() {
            div { class: "sr-frame-strip", role: "group", aria_label: copy.frame_strip_label,
                for (idx, frame) in live_frames.iter().enumerate() {
                    button {
                        r#type: "button",
                        class: "sr-frame-thumb",
                        aria_label: "{frame.alt_text()}",
                        aria_pressed: if *active_frame.read() == idx { "true" } else { "false" },
                        onclick: move |_| active_frame.set(idx),
                        img { src: "{frame.url}", alt: "{frame.alt_text()}", loading: "lazy" }
                    }
                }
            }
        }

        match &*videos.read() {
            None => rsx! {
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(payload)) if payload.videos.is_empty() => rsx! {
                p { class: "sr-status", role: "status", "{copy.empty}" }
            },
            Some(Ok(payload)) => rsx! {
                ul { class: "sr-list", aria_label: copy.tv_title,
                    for video in payload.videos.iter() {
                        TvVideoRow { video: video.clone() }
                    }
                }
            },
        }
    }
}

#[component]
fn TvVideoRow(video: TvVideo) -> Element {
    let status = video.status.as_deref().unwrap_or("ready");
    rsx! {
        li { class: "sr-list-row",
            span { class: "sr-tabular", "{status}" }
            p { class: "sr-row-title", "{video.title}" }
            span { class: "sr-tabular", "{video.id}" }
        }
    }
}

#[component]
fn MangaView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut double_spread = use_signal(|| true);
    let mut page_index = use_signal(|| 0usize);
    let frames = use_server_future(move || load_live_frames(Some("manga".to_owned()), 20))?;

    rsx! {
        document::Title { "{copy.manga_title} — Shenron" }
        document::Meta { name: "description", content: copy.manga_lead }

        h1 { class: "sr-title", "{copy.manga_title}" }
        p { class: "sr-lead", "{copy.manga_lead}" }

        div { class: "sr-toolbar",
            div { class: "sr-control-group", role: "radiogroup", aria_label: copy.manga_title,
                button {
                    r#type: "button",
                    class: "sr-chip-btn",
                    role: "radio",
                    aria_checked: if *double_spread.read() { "true" } else { "false" },
                    onclick: move |_| double_spread.set(true),
                    "{copy.manga_spread_btn}"
                }
                button {
                    r#type: "button",
                    class: "sr-chip-btn",
                    role: "radio",
                    aria_checked: if !*double_spread.read() { "true" } else { "false" },
                    onclick: move |_| double_spread.set(false),
                    "{copy.manga_single_btn}"
                }
            }
        }

        match &*frames.read() {
            None => rsx! {
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(payload)) if payload.frames.is_empty() => rsx! {
                p { class: "sr-status", role: "status", "{copy.empty}" }
            },
            Some(Ok(payload)) => {
                let is_spread = *double_spread.read();
                let current = (*page_index.read()).min(payload.frames.len().saturating_sub(1));
                let first = payload.frames.get(current);
                let second = if is_spread { payload.frames.get(current + 1) } else { None };
                let spread_class = if is_spread && second.is_some() {
                    "sr-spread"
                } else {
                    "sr-spread sr-spread-single"
                };

                rsx! {
                    section { class: "{spread_class}", aria_label: copy.manga_title,
                        if let Some(p1) = first {
                            img { src: "{p1.url}", alt: "{p1.alt_text()}" }
                        }
                        if let Some(p2) = second {
                            img { src: "{p2.url}", alt: "{p2.alt_text()}" }
                        }
                    }

                    div { class: "sr-frame-strip", role: "group", aria_label: copy.frame_strip_label,
                        for (idx, frame) in payload.frames.iter().enumerate() {
                            button {
                                r#type: "button",
                                class: "sr-frame-thumb",
                                aria_label: "{frame.alt_text()}",
                                aria_pressed: if current == idx { "true" } else { "false" },
                                onclick: move |_| page_index.set(idx),
                                img { src: "{frame.url}", alt: "{frame.alt_text()}", loading: "lazy" }
                            }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn GamesView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let frames = use_server_future(move || load_live_frames(Some("games".to_owned()), 12))?;

    let live_frames: Vec<LiveFrame> = match &*frames.read() {
        Some(Ok(payload)) => payload.frames.clone(),
        _ => Vec::new(),
    };

    let catalog = [
        (
            "Dragon Ball Z: Budokai Tenkaichi 3",
            "WASM / WebGPU",
            "Arènes 3D, bibliothèque de personnages et rendu natif calibré pour la latence minimale.",
        ),
        (
            "Dragon Ball Xenoverse 2",
            "CPK / RE Pipeline",
            "Exploration des archives, tenues et séquences extraites en direct par le moteur natif.",
        ),
        (
            "Dragon Pixel",
            "Bevy 0.19 · WebGPU",
            "Moteur de combat déterministe Rust compilé pour navigateur et bureau.",
        ),
    ];

    rsx! {
        document::Title { "{copy.games_title} — Shenron" }
        document::Meta { name: "description", content: copy.games_lead }

        h1 { class: "sr-title", "{copy.games_title}" }
        p { class: "sr-lead", "{copy.games_lead}" }

        section { class: "sr-pillars", aria_label: copy.games_title,
            for (idx, (name, runtime, summary)) in catalog.into_iter().enumerate() {
                article { class: "sr-pillar-panel",
                    if let Some(frame) = live_frames.get(idx).or_else(|| live_frames.first()) {
                        img {
                            class: "sr-pillar-media",
                            src: "{frame.url}",
                            alt: "{frame.alt_text()}",
                            loading: "lazy",
                        }
                    }
                    div { class: "sr-pillar-body",
                        h2 { class: "sr-pillar-heading", "{name}" }
                        span { class: "sr-tabular", "{runtime}" }
                        p { class: "sr-muted", "{summary}" }
                    }
                }
            }
        }
    }
}

#[component]
fn EpisodesView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut filter = use_signal(String::new);
    let episodes = use_server_future(move || load_episodes(60, 0, None))?;

    let needle = filter.read().trim().to_ascii_lowercase();

    rsx! {
        document::Title { "{copy.episodes_title} — Shenron" }
        document::Meta { name: "description", content: copy.episodes_lead }

        h1 { class: "sr-title", "{copy.episodes_title}" }
        p { class: "sr-lead", "{copy.episodes_lead}" }

        div { class: "sr-toolbar",
            label { class: "sr-tabular", r#for: "episodes-filter", "{copy.episodes_search_label}" }
            input {
                id: "episodes-filter",
                class: "sr-input",
                r#type: "search",
                value: "{filter}",
                placeholder: copy.episodes_search_placeholder,
                oninput: move |evt| filter.set(evt.value()),
            }
        }

        match &*episodes.read() {
            None => rsx! {
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(page)) => {
                let visible: Vec<&EpisodeRecord> =
                    page.items.iter().filter(|ep| matches_episode_filter(ep, &needle)).collect();
                if visible.is_empty() {
                    rsx! { p { class: "sr-status", role: "status", "{copy.empty}" } }
                } else {
                    rsx! {
                        ul { class: "sr-list", aria_label: copy.episodes_title,
                            for ep in visible {
                                EpisodeRow { episode: ep.clone() }
                            }
                        }
                    }
                }
            },
        }
    }
}

fn matches_episode_filter(ep: &EpisodeRecord, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if ep.title.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if let Some(series) = ep.series.as_deref()
        && series.to_ascii_lowercase().contains(needle)
    {
        return true;
    }
    ep.number.map(|num| num.to_string().contains(needle)).unwrap_or(false)
}

#[component]
fn EpisodeRow(episode: EpisodeRecord) -> Element {
    let slug = episode.route_slug();
    let series = episode.series.as_deref().unwrap_or("Dragon Ball");
    let number_label = episode.number.map(|n| format!("#{n:03}")).unwrap_or_else(|| "—".to_owned());

    rsx! {
        li { class: "sr-list-row",
            span { class: "sr-tabular", "{number_label}" }
            div {
                h2 { class: "sr-row-title",
                    Link { to: Route::EpisodeDetail { slug }, "{episode.title}" }
                }
                if let Some(ja) = episode.title_ja.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "sr-jp", lang: "ja", "{ja}" }
                }
                if let Some(synopsis) = episode.synopsis.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "sr-muted", "{synopsis}" }
                }
            }
            span { class: "sr-tabular", "{series}" }
        }
    }
}

#[component]
fn EpisodeDetailView(locale: Locale, slug: String) -> Element {
    let copy = copy_for(locale);
    let slug_for_fetch = slug.clone();
    let detail = use_server_future(move || load_episode_detail(slug_for_fetch.clone()))?;

    rsx! {
        div { class: "sr-toolbar",
            Link { class: "sr-cta-secondary", to: Route::Episodes {}, "{copy.back_to_episodes}" }
        }

        match &*detail.read() {
            None => rsx! {
                document::Title { "{slug} — Shenron" }
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                document::Title { "{copy.episodes_title} — Shenron" }
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(episode)) => {
                let meta_desc = episode
                    .synopsis
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .unwrap_or(copy.episodes_lead)
                    .to_owned();
                rsx! {
                    document::Title { "{episode.title} — Shenron" }
                    document::Meta { name: "description", content: meta_desc }

                    article { class: "sr-stage-copy",
                        h1 { class: "sr-title", "{episode.title}" }
                        if let Some(ja) = episode.title_ja.as_deref().filter(|s| !s.is_empty()) {
                            p { class: "sr-jp", lang: "ja", "{ja}" }
                        }
                        if let Some(date) = episode.air_date.as_deref().filter(|s| !s.is_empty()) {
                            p { class: "sr-tabular", "{date}" }
                        }
                        if let Some(synopsis) = episode.synopsis.as_deref().filter(|s| !s.is_empty()) {
                            p { class: "sr-lead", "{synopsis}" }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn MoviesView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let movies = use_server_future(move || load_movies(60, 0))?;

    rsx! {
        document::Title { "{copy.movies_title} — Shenron" }
        document::Meta { name: "description", content: copy.movies_lead }

        h1 { class: "sr-title", "{copy.movies_title}" }
        p { class: "sr-lead", "{copy.movies_lead}" }

        match &*movies.read() {
            None => rsx! {
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(page)) if page.items.is_empty() => rsx! {
                p { class: "sr-status", role: "status", "{copy.empty}" }
            },
            Some(Ok(page)) => rsx! {
                ul { class: "sr-list", aria_label: copy.movies_title,
                    for movie in page.items.iter() {
                        MovieRow { movie: movie.clone() }
                    }
                }
            },
        }
    }
}

#[component]
fn MovieRow(movie: MovieRecord) -> Element {
    let release = movie.release_date.as_deref().unwrap_or("—");
    let series = movie.series.as_deref().unwrap_or("Dragon Ball");

    rsx! {
        li { class: "sr-list-row",
            span { class: "sr-tabular", "{release}" }
            div {
                h2 { class: "sr-row-title", "{movie.title}" }
                if let Some(ja) = movie.title_ja.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "sr-jp", lang: "ja", "{ja}" }
                }
                if let Some(synopsis) = movie.synopsis.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "sr-muted", "{synopsis}" }
                }
            }
            span { class: "sr-tabular", "{series}" }
        }
    }
}

#[component]
fn TimelineView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut selected_era = use_signal(|| Option::<String>::None);
    let events = use_server_future(move || load_timeline_events(None))?;

    rsx! {
        document::Title { "{copy.timeline_title} — Shenron" }
        document::Meta { name: "description", content: copy.timeline_lead }

        h1 { class: "sr-title", "{copy.timeline_title}" }
        p { class: "sr-lead", "{copy.timeline_lead}" }

        match &*events.read() {
            None => rsx! {
                p { class: "sr-status", role: "status", aria_live: "polite", "{copy.loading}" }
            },
            Some(Err(err)) => rsx! {
                p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" }
            },
            Some(Ok(payload)) if payload.events.is_empty() => rsx! {
                p { class: "sr-status", role: "status", "{copy.empty}" }
            },
            Some(Ok(payload)) => {
                let eras = distinct_eras(&payload.events);
                let active_era = selected_era.read().clone();
                let visible: Vec<&TimelineEvent> = payload
                    .events
                    .iter()
                    .filter(|ev| active_era.as_deref().is_none_or(|era| ev.era == era))
                    .collect();

                rsx! {
                    div { class: "sr-toolbar", role: "group", aria_label: copy.timeline_title,
                        div { class: "sr-control-group",
                            button {
                                r#type: "button",
                                class: "sr-chip-btn",
                                aria_pressed: if active_era.is_none() { "true" } else { "false" },
                                onclick: move |_| selected_era.set(None),
                                "{copy.timeline_all_eras}"
                            }
                            for era in eras {
                                {
                                    let is_active = active_era.as_deref() == Some(era.as_str());
                                    let era_value = era.clone();
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: "sr-chip-btn",
                                            aria_pressed: if is_active { "true" } else { "false" },
                                            onclick: move |_| selected_era.set(Some(era_value.clone())),
                                            "{era}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    ul { class: "sr-list", aria_label: copy.timeline_title,
                        for event in visible {
                            TimelineRow { event: event.clone() }
                        }
                    }
                }
            },
        }
    }
}

fn distinct_eras(events: &[TimelineEvent]) -> Vec<String> {
    let mut eras = Vec::new();
    for event in events {
        if !event.era.is_empty() && !eras.iter().any(|existing| existing == &event.era) {
            eras.push(event.era.clone());
        }
    }
    eras
}

#[component]
fn TimelineRow(event: TimelineEvent) -> Element {
    rsx! {
        li { class: "sr-list-row",
            span { class: "sr-tabular", "Age {event.age_year}" }
            div {
                h2 { class: "sr-row-title", "{event.title}" }
                if let Some(ja) = event.title_ja.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "sr-jp", lang: "ja", "{ja}" }
                }
                p { class: "sr-muted", "{event.description}" }
            }
            span { class: "sr-tabular", "{event.era}" }
        }
    }
}

#[component]
fn WikiHubView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    rsx! {
        document::Title { "Encyclopédie Dragon Ball FR — Shenron" }
        document::Meta {
            name: "description",
            content: "Portail encyclopédique complet : personnages, sagas, 882 épisodes, films, chronologie, databooks et sources.",
        }

        h1 { class: "sr-title", "Encyclopédie Dragon Ball FR" }
        p { class: "sr-lead", "Accès direct à l’ensemble des sections migrées du plan du site Shenron : épisodes, films, personnages, sagas, chronologie, databooks et sources." }

        section { class: "sr-pillars", aria_label: "Sections de l’encyclopédie",
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Episodes {}, "{copy.episodes_title}" } }
                    p { class: "sr-muted", "{copy.episodes_lead}" }
                }
            }
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Movies {}, "{copy.movies_title}" } }
                    p { class: "sr-muted", "{copy.movies_lead}" }
                }
            }
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Characters {}, "Personnages" } }
                    p { class: "sr-muted", "Fiches canoniques des combattants, techniques, transformations et généalogies." }
                }
            }
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Sagas {}, "Sagas et arcs" } }
                    p { class: "sr-muted", "Les 41 arcs narratifs de Dragon Ball, Z, GT, Super et Daima." }
                }
            }
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Timeline {}, "{copy.timeline_title}" } }
                    p { class: "sr-muted", "{copy.timeline_lead}" }
                }
            }
            article { class: "sr-pillar-panel",
                div { class: "sr-pillar-body",
                    h2 { class: "sr-pillar-heading", Link { to: Route::Databooks {}, "Databooks et sources" } }
                    p { class: "sr-muted", "483 ouvrages officiels Daizenshuu, Chozenshuu et bibliographie Shueisha." }
                }
            }
        }
    }
}

#[component]
fn CatalogPortalView(
    locale: Locale,
    title: &'static str,
    lead: &'static str,
    api_path: &'static str,
    back_to: Route,
) -> Element {
    let _copy = copy_for(locale);
    rsx! {
        document::Title { "{title} — Shenron" }
        document::Meta { name: "description", content: lead }

        div { class: "sr-toolbar",
            Link { class: "sr-cta-secondary", to: back_to, "Retour à l’encyclopédie" }
        }
        h1 { class: "sr-title", "{title}" }
        p { class: "sr-lead", "{lead}" }

        div { class: "sr-status",
            p { class: "sr-muted", "Source API publique Shenron : " }
            a { class: "sr-cta-secondary", href: "{api_path}", target: "_blank", rel: "noopener noreferrer", "{api_path}" }
        }
    }
}

#[component]
fn SearchView(locale: Locale) -> Element {
    let copy = copy_for(locale);
    let mut query = use_signal(String::new);
    let episodes = use_server_future(move || load_episodes(80, 0, None))?;
    let needle = query.read().trim().to_ascii_lowercase();

    rsx! {
        document::Title { "Recherche — Shenron" }
        document::Meta { name: "description", content: "Recherche instantanée dans le catalogue Dragon Ball FR." }

        h1 { class: "sr-title", "Recherche Dragon Ball FR" }
        p { class: "sr-lead", "Filtrez instantanément les épisodes, films, mangas et arcs de l’encyclopédie." }

        div { class: "sr-toolbar",
            label { class: "sr-tabular", r#for: "global-search-input", "{copy.episodes_search_label}" }
            input {
                id: "global-search-input",
                class: "sr-input",
                r#type: "search",
                value: "{query}",
                placeholder: copy.episodes_search_placeholder,
                oninput: move |evt| query.set(evt.value()),
            }
        }

        match &*episodes.read() {
            None => rsx! { p { class: "sr-status", role: "status", "{copy.loading}" } },
            Some(Err(err)) => rsx! { p { class: "sr-status sr-status-error", role: "alert", "{copy.api_unavailable} ({err})" } },
            Some(Ok(page)) => {
                let visible: Vec<&EpisodeRecord> =
                    page.items.iter().filter(|ep| matches_episode_filter(ep, &needle)).take(25).collect();
                if visible.is_empty() {
                    rsx! { p { class: "sr-status", role: "status", "{copy.empty}" } }
                } else {
                    rsx! {
                        ul { class: "sr-list", aria_label: "Résultats de recherche",
                            for ep in visible {
                                EpisodeRow { episode: ep.clone() }
                            }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn AdminView(locale: Locale) -> Element {
    let _copy = copy_for(locale);
    rsx! {
        document::Title { "Administration — Shenron" }
        document::Meta { name: "description", content: "Console d’administration Shenron protégée par Cloudflare Zero Trust Access." }

        h1 { class: "sr-title", "Console d’administration Shenron" }
        p { class: "sr-lead", "Accès réservé aux opérateurs via Cloudflare Zero Trust Access et jetons de service." }

        div { class: "sr-actions",
            a { class: "sr-cta", href: "https://admin.dragonballfr.com/", "Ouvrir admin.dragonballfr.com" }
            Link { class: "sr-cta-secondary", to: Route::Home {}, "Retour au portail" }
        }
    }
}
