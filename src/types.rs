use clap::Parser;

pub type Score = i32;


/// This program uses the standard flag RUST_LOG to set the log level.
/// Running the program with a log level higher than Info will result in an error.
#[derive(Clone, Parser)]
pub struct Parameters {
    #[arg(short = 'i', long = "iterations", default_value = "200")]
    pub(crate) iterations: i32,

    // #[arg(short = 'v', long = "verbose")]
    // pub(crate) verbose: bool,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
