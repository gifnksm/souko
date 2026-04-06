use std::io;

use clap::CommandFactory as _;
use clap_complete::{Generator, Shell};
use color_eyre::eyre::{Result, WrapErr as _, eyre};

use crate::{
    application::usecase::Usecases,
    presentation::context::{SubcommandContext, global::GlobalContext},
    project_dirs::ProjectDirs,
};

use self::args::Args;

mod message;

pub(crate) mod args;
mod command;
mod config;
mod context;
mod model;
mod render;

pub(crate) fn print_completion(bin_name: &str, shell: &str) -> Result<()> {
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
        _ => {
            bail!(eyre!(
                "unknown shell `{shell}`, expected one of `bash`, `elvish`, `fish`, `powershell`, `zsh`, `nushell`"
            ));
        }
    }
    Ok(())
}

pub(crate) fn generate_man(output_dir: &str) -> Result<()> {
    clap_mangen::generate_to(Args::command(), output_dir)
        .wrap_err_with(|| format!("failed to generate man pages in {output_dir}"))?;
    Ok(())
}

pub(crate) fn dispatch(args: &Args, usecases: Usecases, project_dirs: ProjectDirs) -> Result<()> {
    let Some(subcommand) = args.subcommand() else {
        Args::command().print_help()?;
        return Ok(());
    };
    let global_ctx = GlobalContext::new(args, usecases, project_dirs)?;
    let subcommand_ctx = SubcommandContext::new(&global_ctx, subcommand)?;
    command::dispatch(&global_ctx, &subcommand_ctx)
}
