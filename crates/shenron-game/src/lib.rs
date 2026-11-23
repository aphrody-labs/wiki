// SPDX-License-Identifier: Apache-2.0
//! Clean-room Bandai / Unreal / Steam virtual asset & ECS foundation for Shenron.
//!
//! Maps Unreal-style virtual mount points (`/Game/DBXV2`, `/Game/DBFZ`) directly
//! onto the real-time `fr1` XFS CDN (`http://10.200.77.1:8800`) so Bevy/Dioxus
//! runtimes stream CPK and UE4 PAK assets on demand without copying game binaries.

use serde::{Deserialize, Serialize};

pub const DEFAULT_GAMES_CDN_ORIGIN: &str = "http://10.200.77.1:8800";
pub const DBXV2_STEAM_APP_ID: u32 = 454_650;
pub const DBFZ_STEAM_APP_ID: u32 = 678_950;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CleanRoomFormat {
    DimpsCpk,
    ArcSysUe4Pak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualMount {
    pub virtual_root: &'static str,
    pub slug: &'static str,
    pub steam_app_id: u32,
    pub primary_depot_id: u32,
    pub format: CleanRoomFormat,
}

pub const MOUNTS: [VirtualMount; 2] = [
    VirtualMount {
        virtual_root: "/Game/DBXV2",
        slug: "dbxv2",
        steam_app_id: DBXV2_STEAM_APP_ID,
        primary_depot_id: 454_651,
        format: CleanRoomFormat::DimpsCpk,
    },
    VirtualMount {
        virtual_root: "/Game/DBFZ",
        slug: "dbfz",
        steam_app_id: DBFZ_STEAM_APP_ID,
        primary_depot_id: 678_951,
        format: CleanRoomFormat::ArcSysUe4Pak,
    },
];

/// Resolve an Unreal-style virtual path (`/Game/DBXV2/...` or `/Game/DBFZ/...`)
/// to its live `fr1` XFS CDN URL.
pub fn resolve_virtual_url(virtual_path: &str, origin: &str) -> Option<String> {
    let origin = origin.trim_end_matches('/');
    for mount in MOUNTS {
        if let Some(rest) = virtual_path.strip_prefix(mount.virtual_root) {
            let rel = rest.trim_start_matches('/');
            if rel.is_empty() || rel.contains("..") {
                return None;
            }
            return Some(format!("{origin}/games/{}/{rel}", mount.slug));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_dbxv2_and_dbfz_virtual_paths() {
        assert_eq!(
            resolve_virtual_url("/Game/DBXV2/data/data.cpk", DEFAULT_GAMES_CDN_ORIGIN),
            Some("http://10.200.77.1:8800/games/dbxv2/data/data.cpk".to_string())
        );
        assert_eq!(
            resolve_virtual_url(
                "/Game/DBFZ/RED/Content/Paks/RED-WindowsNoEditor.pak",
                DEFAULT_GAMES_CDN_ORIGIN
            ),
            Some(
                "http://10.200.77.1:8800/games/dbfz/RED/Content/Paks/RED-WindowsNoEditor.pak"
                    .to_string()
            )
        );
        assert_eq!(
            resolve_virtual_url("/Game/DBXV2/../secret", DEFAULT_GAMES_CDN_ORIGIN),
            None
        );
    }
}
