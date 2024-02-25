use clap::Parser;

pub type Score = i32;


#[derive(Clone, Parser)]
pub struct Parameters {
    #[arg(short = 'i', long = "iterations", default_value = "200")]
    pub(crate) iterations: i32,

    // #[arg(short = 'v', long = "verbose")]
    // pub(crate) verbose: bool,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
