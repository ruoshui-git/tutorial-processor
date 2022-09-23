use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Dump file metadata
    Meta { file: PathBuf },
    Concat {
        /// Files to be prepended to the lecture video
        #[clap(short, long, value_parser, default_value = "片头.mp4")]
        intro: Vec<PathBuf>,

        /// Files to be appended to the lecture video
        #[clap(short, long, value_parser, default_value = "片尾.mp4")]
        outro: Vec<PathBuf>,

        /// Lecture files (each one will be processed individually)
        #[clap(default_value = ".")]
        lecture: Vec<PathBuf>,

        /// Suffix to add to file names after processing
        #[clap(short, long, default_value = " 处理后")]
        suffix: String,
    },
}
