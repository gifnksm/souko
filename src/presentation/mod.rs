use std::{ffi::OsString, io};

use clap::{CommandFactory as _, Parser as _};
use clap_complete::{Generator, Shell};
pub use color_eyre::eyre::{Result, WrapErr as _};

use self::args::Args;
use crate::{application::service::Service, project_dirs::ProjectDirs};

mod args;
mod config;
mod model;

#[derive(Debug)]
pub(crate) struct Presentation {
    args: Args,
    project_dirs: ProjectDirs,
}

impl Presentation {
    pub(crate) fn from_args<I, T>(project_dirs: ProjectDirs, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args = Args::parse_from(args);
        Self { args, project_dirs }
    }

    pub(crate) fn main(self, service: &Service) -> Result<()> {
        let Self { args, project_dirs } = self;
        args.run(service, &project_dirs)?;
        Ok(())
    }

    pub(crate) fn verbosity(&self) -> Option<tracing::Level> {
        self.args.verbosity()
    }

    pub(crate) fn print_completion(bin_name: &str, shell: &str) {
        fn generate_completion<G>(bin_name: &str, g: G)
        where
            G: Generator,
        {
            clap_complete::generate(g, &mut Args::command(), bin_name, &mut io::stdout());
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

    pub(crate) fn generate_man(output_dir: &str) -> Result<()> {
        clap_mangen::generate_to(Args::command(), output_dir)
            .wrap_err_with(|| format!("failed to generate man pages in {output_dir}"))?;
        Ok(())
    }
}
