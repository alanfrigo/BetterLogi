//! Where BetterLogi's update feeds live on the update host.
//!
//! The host is the repository's GitHub Releases (`…/releases`): every feed is
//! an asset of the newest release, so it is served from `latest/download/`. The app reads the
//! built-in engine's manifest from it, packaging stamps the Sparkle feed URL
//! from it, and xtask writes both feeds for it. All of them derive the
//! location here, so the URL an installed app reads and the feed xtask writes
//! for that URL cannot drift apart. The release workflow's upload
//! destinations (`release.yml`) still spell the layout themselves.

use serde::{Deserialize, Serialize};

/// A release channel: the set of releases an installation is offered.
///
/// Settings persist the channel a user picks by its name, for example
/// `update_channel = "beta"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateChannel {
    /// Final releases, tagged `vX.Y.Z`.
    Stable,
    /// Pre-releases, tagged `vX.Y.Z-beta.N`. Nothing publishes to it yet.
    Beta,
}

impl UpdateChannel {
    /// Every channel, in the order Settings offers them.
    pub const ALL: [Self; 2] = [Self::Stable, Self::Beta];

    /// The channel an installation follows: `chosen`, the one the user picked,
    /// or else the channel of `running_version`. Without a pick, a pre-release
    /// such as `0.8.17-beta.12` follows beta and a final release follows
    /// stable, so an unpicked beta that updates to a final release follows
    /// stable from then on. A picked channel stays until the user changes it.
    /// Every reader of the channel decides it here.
    #[must_use]
    pub fn followed(chosen: Option<Self>, running_version: &str) -> Self {
        chosen.unwrap_or_else(|| Self::of_version(running_version))
    }

    /// The channel `version` comes from: a semantic-version pre-release is a
    /// beta. The pre-release starts at the first `-` before any `+`, because
    /// build metadata may contain `-` too.
    fn of_version(version: &str) -> Self {
        let without_build = version.split_once('+').map_or(version, |(head, _)| head);
        if without_build.contains('-') {
            Self::Beta
        } else {
            Self::Stable
        }
    }

    /// The channel's name, as the feed layout and the feeds themselves spell
    /// it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
        }
    }

    /// URL of this channel's feed directory on the GitHub Releases host at
    /// `base_url`. Every feed of the channel is a file directly inside it.
    /// Trailing slashes on `base_url` are dropped.
    // ponytail: GitHub's `latest` skips pre-releases, so beta reads the newest
    // stable feeds (the release workflow publishes nothing else yet). Give beta
    // its own location once betas publish.
    #[must_use]
    pub fn dir_url(self, base_url: &str) -> String {
        format!("{}/latest/download", base_url.trim_end_matches('/'))
    }

    /// URL of the release `tag`'s assets on the GitHub Releases host at
    /// `base_url`: where every feed points its downloads.
    #[must_use]
    pub fn release_url(base_url: &str, tag: &str) -> String {
        format!("{}/download/{tag}", base_url.trim_end_matches('/'))
    }

    /// URL of the built-in update engine's manifest for this channel on the
    /// update host at `base_url`.
    #[must_use]
    pub fn manifest_url(self, base_url: &str) -> String {
        format!("{}/latest.json", self.dir_url(base_url))
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateChannel;

    #[test]
    fn feeds_and_assets_live_on_github_releases() {
        let base = "https://github.com/alanfrigo/BetterLogi/releases/";
        assert_eq!(
            UpdateChannel::Stable.manifest_url(base),
            "https://github.com/alanfrigo/BetterLogi/releases/latest/download/latest.json"
        );
        assert_eq!(
            UpdateChannel::release_url(base, "v1.0.0"),
            "https://github.com/alanfrigo/BetterLogi/releases/download/v1.0.0"
        );
    }

    #[test]
    fn a_chosen_channel_wins_over_the_running_version() {
        for version in ["0.8.16", "0.8.17-beta.12"] {
            for channel in UpdateChannel::ALL {
                assert_eq!(UpdateChannel::followed(Some(channel), version), channel);
            }
        }
    }

    #[test]
    fn without_a_choice_a_pre_release_follows_beta() {
        for (version, channel) in [
            ("0.8.16", UpdateChannel::Stable),
            ("0.8.16+build-7", UpdateChannel::Stable),
            ("0.8.17-beta.12", UpdateChannel::Beta),
            ("0.8.17-beta.12+build-7", UpdateChannel::Beta),
        ] {
            assert_eq!(UpdateChannel::followed(None, version), channel, "{version}");
        }
    }
}
