use std::iter::FromIterator;

use clap::Parser;
use eyre::{eyre, Context, Result};
use log::{info, warn};
use tpc::{
    cli::{Cli, Commands},
    concat::concat,
    meta,
};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let cli = Cli::parse();
    pretty_env_logger::env_logger::Builder::new()
        // .filter_level(args.verbose.log_level_filter())
        .filter_module("tpc", cli.verbose.log_level_filter())
        .init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Meta { file }) => {
            meta::dump_meta(file)
            // println!("'myapp add' was used, name is: {:?}", name)
        }
        Some(Commands::Concat {
            intro,
            outro,
            lecture,
            suffix,
        }) => {
            for file in intro {
                if !file.exists() {
                    return Err(eyre!("File {file:?} does not exist"));
                }
            }
            for file in outro {
                if !file.exists() {
                    return Err(eyre!("File {file:?} does not exist"));
                }
            }

            let media: Vec<_> = lecture
                .iter()
                .flat_map(|entry| WalkDir::new(entry).max_depth(1).into_iter())
                .filter_map(|entry| entry.ok())
                .filter(|good_entry| good_entry.file_type().is_file())
                .filter_map(|file| match infer::get_from_path(file.path()) {
                    Ok(op) => match op {
                        Some(t) => {
                            let mime = t.mime_type();
                            if mime.starts_with("video") || mime.starts_with("image") {
                                Some(file)
                            } else {
                                None
                            }
                        }
                        None => {
                            warn!("File type unknown for file `{:?}`", file.path());
                            None
                        }
                    },
                    Err(e) => {
                        warn!("Failed to read path {:?}; Error: {:?}", file.path(), e);
                        None
                    }
                })
                .map(|entry| entry.into_path())
                .collect();

            info!("media files: {media:?}");
            info!("suffix: {suffix}");

            for file in media {
                concat(&[&intro[..], &[file.clone()], &outro[..]].concat())
                    .wrap_err_with(|| format!("Error concat-ing {file:?}"))?;
            }
            Ok(())
        }
        None => {
            todo!("Default actions");
        }
    }
}
