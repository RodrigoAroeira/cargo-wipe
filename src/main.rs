use std::io::stdout;

use clap::Parser;
use strum::IntoEnumIterator;

mod command;
mod dir_helpers;
mod language;
mod wipe;
mod wipe_params;
mod writer;

use crate::command::Args;
use crate::language::Language;
use crate::wipe::Wipe;
use crate::wipe_params::WipeParams;

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let stdout = stdout();
    let mut args = Args::parse();
    const ALL: Language = Language::All;
    let languages = match args.language {
        ALL => Language::iter().filter(|&l| l != ALL).collect(),
        l => vec![l],
    };

    for l in languages {
        args.language = l;
        let params = WipeParams::new(&args)?;
        if let Err(e) = Wipe::new(&mut stdout.lock(), &params).run() {
            eprintln!("An error occurred: {e}")
        }
    }

    Ok(())
}
