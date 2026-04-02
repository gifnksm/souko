use std::{env, io, process};

use clap::{CommandFactory as _, Parser as _};
use clap_complete::{Generator, Shell};
pub use color_eyre::eyre::Result;

use self::{args::Args, model::project_dirs::ProjectDirs};
use crate::application::service::Service;

mod args;
mod config;
mod model;

#[derive(Debug)]
pub(crate) struct Presentation {
    args: Args,
    project_dirs: ProjectDirs,
}

impl Presentation {
    pub(crate) fn from_env(bin_name: &str) -> Result<Self> {
        let env_prefix = bin_name.to_uppercase().replace("-", "_");
        if let Ok(shell) = env::var(format!("{env_prefix}_PRINT_COMPLETION")) {
            Self::print_completion(bin_name, &shell);
            process::exit(0);
        }
        if let Ok(output_dir) = env::var(format!("{env_prefix}_GENERATE_MAN_TO")) {
            Self::generate_man(&output_dir);
            process::exit(0);
        }

        let args = Args::parse();

        let project_dirs = ProjectDirs::new()?;
        Ok(Self { args, project_dirs })
    }

    pub(crate) fn command() -> clap::Command {
        Args::command()
    }

    pub(crate) fn main(self, service: &Service) -> Result<()> {
        let Self { args, project_dirs } = self;
        args.run(service, &project_dirs)?;
        Ok(())
    }

    pub(crate) fn verbosity(&self) -> Option<tracing::Level> {
        self.args.verbosity()
    }

    fn print_completion(bin_name: &str, shell: &str) {
        fn generate_completion<G>(bin_name: &str, g: G)
        where
            G: Generator,
        {
            clap_complete::generate(g, &mut Presentation::command(), bin_name, &mut io::stdout());
        }
        match shell {
            "bash" => generate_completion(bin_name, Shell::Bash),
            "elvish" => generate_completion(bin_name, Shell::Elvish),
            "fish" => generate_completion(bin_name, Shell::Fish),
            "powershell" => generate_completion(bin_name, Shell::PowerShell),
            "zsh" => generate_completion(bin_name, Shell::Zsh),
            "nushell" => generate_completion(bin_name, clap_complete_nushell::Nushell),
            _ => panic!(
                "error: unknown shell `{shell}`, expected one of `bash`, `elvish`, `fish`, `powershell`, `zsh`, `nushell`"
            ),
        }
    }

    fn generate_man(output_dir: &str) {
        clap_mangen::generate_to(Self::command(), output_dir).unwrap();
    }
}
