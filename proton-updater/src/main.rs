#[cfg(not(all(target_os = "linux")))]
compile_error!("only supportsed on linux");

mod proton_builds;

use anyhow::{Result, bail};
use clap::Parser;
use proton_builds::{ProtonBuild, ProtonBuildId};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    /// Proton build identifier to manage.
    #[clap(long = "build", value_enum)]
    pub build: ProtonBuildId,

    /// Base directory to use when managing the proton installation.
    ///
    /// A version file will be placed in the root of the directory and a subdirectory named "proton" will be created.
    #[clap(long = "install-dir-base")]
    pub install_dir_base: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.install_dir_base.exists() {
        bail!("Install base directory does not exist - please create it before running.")
    };

    args.build
        .get_build()
        .install_or_update(
            args.install_dir_base.join("proton"),
            args.install_dir_base.join(".version"),
            args.install_dir_base.join(".proton-tmp"),
        )
        .await
}
