use std::io::{self, Write};

use color_eyre::eyre::Result;
use serde::ser::{SerializeSeq, Serializer};

use crate::{cli::subcommand::list::Args, repo::Repo, walk_repo::WalkRepo, App};

pub(super) fn run(app: &App, args: &Args) -> Result<()> {
    let config = app.config()?;
    let root_path = config.root_path();

    let walk_dir = walkdir::WalkDir::new(root_path.value());
    let walk_repo = WalkRepo::new(walk_dir);
    let repos = walk_repo.into_iter().filter_map(|res| match res {
        Ok(res) => Some(res),
        Err(e) => {
            tracing::warn!(error = %e, "failed to traverse directory");
            None
        }
    });

    if args.json() {
        emit_json(repos)?;
    } else {
        emit_text(repos)?;
    }

    Ok(())
}

fn emit_json(repos: impl Iterator<Item = Repo>) -> Result<()> {
    let out = io::stdout();
    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(None)?;

    for repo in repos {
        seq.serialize_element(&repo)?;
    }

    seq.end()?;
    let mut out = ser.into_inner();
    out.flush()?;

    Ok(())
}

fn emit_text(repos: impl Iterator<Item = Repo>) -> Result<()> {
    for repo in repos {
        println!("{}", repo.path().display());
    }
    Ok(())
}
