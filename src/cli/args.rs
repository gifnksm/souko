use clap::ArgAction;
use tracing::Level;

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct Verbosity {
    /// More output per occurrence
    #[clap(long, short = 'v', action = ArgAction::Count, global = true)]
    verbose: i8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: i8,
}

impl Verbosity {
    pub(crate) fn verbosity(&self) -> Option<Level> {
        let level = self.verbose - self.quiet;
        match level {
            i8::MIN..=-3 => None,
            -2 => Some(Level::ERROR),
            -1 => Some(Level::WARN),
            0 => Some(Level::INFO),
            1 => Some(Level::DEBUG),
            2..=i8::MAX => Some(Level::TRACE),
        }
    }
}
