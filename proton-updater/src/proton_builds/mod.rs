mod proton_ge;

use anyhow::Result;
use clap::ValueEnum;
use proton_ge::ProtonGE;
use std::path::PathBuf;

pub trait ProtonBuild {
    async fn install_or_update(
        &self,
        proton_dir_path: PathBuf,
        version_file_path: PathBuf,
        temp_dir_path: PathBuf,
    ) -> Result<()>;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProtonBuildId {
    ProtonGE,
}

impl ProtonBuildId {
    pub fn get_build(&self) -> impl ProtonBuild {
        match self {
            Self::ProtonGE => ProtonGE::default(),
        }
    }
}
