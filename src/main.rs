#[cfg(not(all(target_os = "linux")))]
compile_error!("only supportsed on linux");

use anyhow::{Context, Result};
use bytes::Buf;
use clap::Parser;
use flate2::bufread::GzDecoder;
use std::{fs, path::PathBuf};
use tar::Archive;

const PROTON_FOLDER_NAME: &str = "proton";
const PROTON_ENTRYPOINT_NAME: &str = "proton";
const VERSION_FILE_NAME: &str = ".version";

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    /// Name of the GitHub repository to fetch proton releases from.
    #[clap(long = "repo-name")]
    pub repo_name: String,

    /// Owner of the GitHub repository to fetch proton releases from.
    #[clap(long = "repo-owner")]
    pub repo_owner: String,

    /// The name of proton release tarball.
    ///
    /// Matched losely - for example: a value of "GE-Proton" will match "GE-Proton*"
    /// Please note that the tarball asset must in the .tar.gz format.
    #[clap(long = "tarball-name")]
    pub tarball_name: String,

    /// The base directory to use for managing the proton installation.
    ///
    /// A version file will be placed in the root of the directory and a subdirectory named "proton" will be created.
    #[clap(long = "install-dir-base")]
    pub install_dir_base: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let version_file_path = args.install_dir_base.join(VERSION_FILE_NAME);
    let proton_directory = args.install_dir_base.join(PROTON_FOLDER_NAME);
    let proton_entrypoint = proton_directory.join(PROTON_ENTRYPOINT_NAME);

    // Check if we need to update proton.
    let current_version = fs::read_to_string(&version_file_path).unwrap_or_default();
    let release = octocrab::instance()
        .repos(&args.repo_owner, &args.repo_name)
        .releases()
        .get_latest()
        .await
        .context(format!(
            "failed to get latest release from {}/{}",
            args.repo_owner, args.repo_name
        ))?;

    // Update proton
    let entrypoint_exists = proton_entrypoint.exists();
    if !entrypoint_exists || current_version != release.tag_name {
        println!(
            "Proton is out of date (current: {current_version}, latest: {}, entrypoint exists: {})",
            release.tag_name, entrypoint_exists
        );
        println!("Downloading new version of proton");

        // Download and create the proton archive.
        let bytes = reqwest::get(
            release
                .assets
                .into_iter()
                .find(|asset| {
                    asset.name.contains(&args.tarball_name) && asset.name.ends_with(".tar.gz")
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
        let tmp_install_dir = args.install_dir_base.join("proton-install-tmp");
        let _ = fs::remove_dir_all(&tmp_install_dir);

        // Unpack tar to temp dir incase of errors.
        archive.unpack(&tmp_install_dir)?;

        // Clean old install and replace the proton dir with the temp dir.
        // If there's a leading folder move that instead of the temp dir.
        let _ = fs::remove_file(&version_file_path);
        let _ = fs::remove_dir_all(&proton_directory);
        let entries = fs::read_dir(&tmp_install_dir)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        if entries.len() == 1 && entries[0].path().is_dir() {
            fs::rename(&entries[0].path(), &proton_directory)?;
            let _ = fs::remove_dir_all(&tmp_install_dir);
        } else {
            fs::rename(&tmp_install_dir, &proton_directory)?;
        }
        fs::write(&version_file_path, &release.tag_name)?;

        println!("Proton was successfully updated to {}", release.tag_name);
    } else {
        println!("Proton is already up to date")
    };

    Ok(())
}
