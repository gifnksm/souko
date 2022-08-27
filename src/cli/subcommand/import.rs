use std::path::PathBuf;

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct Args {
    /// Recursively import any subdirectories found
    #[clap(short, long)]
    recursive: bool,
    /// Only print the paths of the repositories that would be imported
    #[clap(short = 'n', long)]
    dry_run: bool,
    /// Also import repositories under hidden directories
    #[clap(long)]
    hidden: bool,
    /// Path of local repositories to import into souko
    #[clap(required = true)]
    repos: Vec<PathBuf>,
}

impl Args {
    pub(crate) fn recursive(&self) -> bool {
        self.recursive
    }

    pub(crate) fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub(crate) fn hidden(&self) -> bool {
        self.hidden
    }

    pub(crate) fn repos(&self) -> &[PathBuf] {
        &self.repos
    }
}
