use super::ProtonBuild;
use anyhow::Context;
use async_trait::async_trait;
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
use tar::Archive;
use zip::ZipArchive;

#[derive(Debug)]
pub struct ProtonTkg {
    github_repo_name: &'static str,
    github_repo_owner: &'static str,
    proton_entrypoint: &'static str,
    proton_artifact_name: &'static str,
    proton_workflow_name: &'static str,
}

impl ProtonTkg {
    pub fn wine_master() -> Self {
        Self {
            github_repo_name: "wine-tkg-git",
            github_repo_owner: "Frogging-Family",
            proton_entrypoint: "proton",
            proton_artifact_name: "proton-tkg-build.zip",
            proton_workflow_name: "proton-arch-nopackage.yml",
        }
    }

    pub fn valve_be() -> Self {
        Self {
            github_repo_name: "wine-tkg-git",
            github_repo_owner: "Frogging-Family",
            proton_entrypoint: "proton",
            proton_artifact_name: "proton-tkg-build.zip",
            proton_workflow_name: "proton-valvexbe-arch-nopackage.yml",
        }
    }
}

#[async_trait]
impl ProtonBuild for ProtonTkg {
    async fn install_or_update(
        &self,
        proton_dir_path: PathBuf,
        version_file_path: PathBuf,
        temp_dir_path: PathBuf,
    ) -> anyhow::Result<()> {
        let proton_entrypoint_path = proton_dir_path.join(self.proton_entrypoint);
        let current_version = fs::read_to_string(&version_file_path).unwrap_or_default();

        // Get the latest run artifact download url and run id.
        // Bypass the github token requirement for artifacts by using nightly.link.
        let (download_url, proton_build_id) = {
            let octocrab = octocrab::instance();
            let run = octocrab
                .workflows(self.github_repo_owner, self.github_repo_name)
                .list_runs(self.proton_workflow_name)
                .status("success")
                .branch("master")
                .send()
                .await?;
            let run = run
                .items
                .first()
                .context("failed to get workflow run that was successful from main branch")?;

            println!("Got run: {}:{}", run.name, run.id);
            (
                format!(
                    "https://nightly.link/{}/{}/actions/runs/{}/{}",
                    self.github_repo_owner,
                    self.github_repo_name,
                    run.id,
                    self.proton_artifact_name
                ),
                run.id.0.to_string(),
            )
        };

        let entrypoint_exists = proton_entrypoint_path.exists();
        if !entrypoint_exists || current_version != proton_build_id {
            println!(
                "Proton-Tkg is out of date (current: {current_version}, latest: {}, entrypoint exists: {})",
                proton_build_id, entrypoint_exists
            );
            println!("Downloading new version of Proton-Tkg");

            let bytes = reqwest::get(download_url)
                .await
                .context("request to download proton artifact failed")?
                .bytes()
                .await?;

            // Read the tar file from the artifact zip. The zip only contains 1 file, so we can assume it's always index 0.
            let mut zip = ZipArchive::new(Cursor::new(bytes))?;
            let mut zip = zip.by_index(0)?;

            let mut tar_buffer = Vec::with_capacity(zip.size() as usize);
            zip.read_to_end(&mut tar_buffer)?;
            let mut archive = Archive::new(Cursor::new(tar_buffer));

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
            fs::write(&version_file_path, &proton_build_id)?;

            println!("Proton-Tkg was successfully updated to {}", proton_build_id);
        } else {
            println!("Proton-Tkg is already up to date")
        }

        Ok(())
    }
}
