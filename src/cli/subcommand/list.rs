#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct Args {
    #[clap(long)]
    json: bool,
}

impl Args {
    pub(crate) fn json(&self) -> bool {
        self.json
    }
}
