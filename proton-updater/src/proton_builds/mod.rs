mod proton_cachyos;
mod proton_ge;
mod proton_tkg;

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
use proton_cachyos::ProtonCachyOS;
use proton_ge::ProtonGE;
use proton_tkg::ProtonTkg;
use std::path::PathBuf;

#[async_trait]
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
    ProtonTkgWine,
    ProtonTkgValvebe,
    ProtonCachyOS,
    ProtonCachyOSOptimised,
}

impl ProtonBuildId {
    pub fn get_build(&self) -> Box<dyn ProtonBuild> {
        match self {
            Self::ProtonGE => Box::new(ProtonGE::default()),
            Self::ProtonTkgWine => Box::new(ProtonTkg::wine_master()),
            Self::ProtonTkgValvebe => Box::new(ProtonTkg::valve_be()),
            Self::ProtonCachyOS => Box::new(ProtonCachyOS::standard()),
            Self::ProtonCachyOSOptimised => Box::new(ProtonCachyOS::optimised()),
        }
    }
}
