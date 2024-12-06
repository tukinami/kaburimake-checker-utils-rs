use std::path::PathBuf;

use clap::Parser;

const DEFAULT_TARGET_PATH: &str = "./ghost_list.json";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) enum Cli {
    /// append ghost data from directories to a json.
    Append(AppendArgs),
    /// build ghost data json from directories.
    Build(BuildArgs),
    /// erase ghost data from json.
    Erase(EraseArgs),
    /// merge ghost data jsons.
    Merge(MergeArgs),
}

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub(crate) struct AppendArgs {
    /// paths to installed ghost directory. e.g. C:/SSP/ghost.
    #[arg(short, long, value_name = "DIRS")]
    input: Vec<PathBuf>,
    /// output path.
    #[arg(short, long, value_name = "PATH", default_value = DEFAULT_TARGET_PATH)]
    output: PathBuf,
}

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub(crate) struct BuildArgs {
    /// paths to installed ghost directory. e.g. C:/SSP/ghost.
    #[arg(short, long, value_name = "DIRS")]
    input: Vec<PathBuf>,
    /// output path.
    #[arg(short, long, value_name = "PATH", default_value = DEFAULT_TARGET_PATH)]
    output: PathBuf,
}

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub(crate) struct EraseArgs {
    /// path to target.
    #[arg(short, long, value_name = "PATH", default_value = DEFAULT_TARGET_PATH)]
    target: PathBuf,
    /// directory name that you want to erase from target.
    #[arg(short, long, value_name = "DIR")]
    directory: Option<String>,
    /// sakuraName that you want to erase from target.
    #[arg(short, long, value_name = "NAME")]
    sakura_name: Option<String>,
    /// keroName that you want to erase from target.
    #[arg(short, long, value_name = "NAME")]
    kero_name: Option<String>,
}

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub(crate) struct MergeArgs {
    /// paths to ghost data jsons.
    #[arg(short, long, value_name = "JSONS")]
    input: Vec<PathBuf>,
    /// output path.
    #[arg(short, long, value_name = "PATH", default_value = DEFAULT_TARGET_PATH)]
    output: PathBuf,
}

impl AppendArgs {
    #[cfg(test)]
    pub fn new(input: Vec<PathBuf>, output: PathBuf) -> AppendArgs {
        AppendArgs { input, output }
    }

    pub fn input(&self) -> &Vec<PathBuf> {
        &self.input
    }
    pub fn output(&self) -> &PathBuf {
        &self.output
    }
}

impl BuildArgs {
    #[cfg(test)]
    pub fn new(input: Vec<PathBuf>, output: PathBuf) -> BuildArgs {
        BuildArgs { input, output }
    }

    pub fn input(&self) -> &Vec<PathBuf> {
        &self.input
    }
    pub fn output(&self) -> &PathBuf {
        &self.output
    }
}

impl EraseArgs {
    #[cfg(test)]
    pub fn new(
        target: PathBuf,
        directory: Option<String>,
        sakura_name: Option<String>,
        kero_name: Option<String>,
    ) -> EraseArgs {
        EraseArgs {
            target,
            directory,
            sakura_name,
            kero_name,
        }
    }

    pub fn target(&self) -> &PathBuf {
        &self.target
    }

    pub fn directory(&self) -> Option<&String> {
        self.directory.as_ref()
    }

    pub fn sakura_name(&self) -> Option<&String> {
        self.sakura_name.as_ref()
    }

    pub fn kero_name(&self) -> Option<&String> {
        self.kero_name.as_ref()
    }
}

impl MergeArgs {
    #[cfg(test)]
    pub fn new(input: Vec<PathBuf>, output: PathBuf) -> MergeArgs {
        MergeArgs { input, output }
    }

    pub fn input(&self) -> &Vec<PathBuf> {
        &self.input
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }
}
