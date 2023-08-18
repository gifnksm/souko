use clap::ArgAction;
use tracing::Level;

#[derive(Debug, Clone, Default, clap::Args)]
pub(super) struct Verbosity {
    /// More output per occurrence
    #[clap(long, short = 'v', action = ArgAction::Count, global = true)]
    verbose: u8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: u8,
}

impl Verbosity {
    pub(super) fn verbosity(&self) -> Option<Level> {
        let level = i8::try_from(self.verbose).unwrap_or(i8::MAX)
            - i8::try_from(self.quiet).unwrap_or(i8::MAX);
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
