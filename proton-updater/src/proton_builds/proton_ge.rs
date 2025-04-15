use super::ProtonBuild;
use anyhow::Context;
use bytes::Buf;
use flate2::bufread::GzDecoder;
use std::{fs, path::PathBuf};
use tar::Archive;

#[derive(Debug)]
pub struct ProtonGE {
    github_repo_name: &'static str,
    github_repo_owner: &'static str,
    proton_entrypoint: &'static str,
    proton_release_asset_name: &'static str,
    proton_release_asset_ext: &'static str,
}

impl Default for ProtonGE {
    fn default() -> Self {
        Self {
            github_repo_name: "proton-ge-custom",
            github_repo_owner: "GloriousEggroll",
            proton_entrypoint: "proton",
            proton_release_asset_name: "GE-Proton",
            proton_release_asset_ext: ".tar.gz",
        }
    }
}

impl ProtonBuild for ProtonGE {
    async fn install_or_update(
        &self,
        proton_dir_path: PathBuf,
        version_file_path: PathBuf,
        temp_dir_path: PathBuf,
    ) -> anyhow::Result<()> {
        let proton_entrypoint_path = proton_dir_path.join(self.proton_entrypoint);

        let current_version = fs::read_to_string(&version_file_path).unwrap_or_default();
        let release = octocrab::instance()
            .repos(self.github_repo_owner, self.github_repo_name)
            .releases()
            .get_latest()
            .await
            .context(format!(
                "failed to get latest release from {}/{}",
                self.github_repo_owner, self.github_repo_name
            ))?;
        let entrypoint_exists = proton_entrypoint_path.exists();
        if !entrypoint_exists || current_version != release.tag_name {
            println!(
                "Proton-GE is out of date (current: {current_version}, latest: {}, entrypoint exists: {})",
                release.tag_name, entrypoint_exists
            );
            println!("Downloading new version of Proton-GE");
            let bytes = reqwest::get(
                release
                    .assets
                    .into_iter()
                    .find(|asset| {
                        asset.name.contains(self.proton_release_asset_name)
                            && asset.name.ends_with(self.proton_release_asset_ext)
                    })
                    .context("no tarball containing a proton release found")?
                    .browser_download_url,
            )
            .await
            .context("request to download proton tarball failed")?
            .bytes()
            .await?;

            let mut archive = Archive::new(GzDecoder::new(bytes.reader()));

            // Cleanup any previous failed installs.
            let _ = fs::remove_dir_all(&temp_dir_path);

            // Unpack tar to temp dir incase of errors.
            archive.unpack(&temp_dir_path)?;

            // Clean old install and replace the proton dir with the temp dir.
            // If there's a leading folder move that instead of the temp dir.
            let _ = fs::remove_file(&version_file_path);
            let _ = fs::remove_dir_all(&proton_dir_path);
            let entries = fs::read_dir(&temp_dir_path)?
                .filter_map(Result::ok)
                .collect::<Vec<_>>();
            if entries.len() == 1 && entries[0].path().is_dir() {
                fs::rename(entries[0].path(), &proton_dir_path)?;
                let _ = fs::remove_dir_all(&temp_dir_path);
            } else {
                fs::rename(&temp_dir_path, &proton_dir_path)?;
            }
            fs::write(&version_file_path, &release.tag_name)?;

            println!("Proton-GE was successfully updated to {}", release.tag_name);
        } else {
            println!("Proton-GE is already up to date")
        }

        Ok(())
    }
}
