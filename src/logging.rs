use std::env;
use log::Level;
use thiserror::Error;

pub fn init() -> anyhow::Result<(), LogLevelError> {
    let log_level_str = env::var("RUST_LOG").map_err(|_| LogLevelError::EnvVarNotSet)?;
    let log_level = log_level_str.parse::<Level>().map_err(|_| LogLevelError::ParseError)?;

    if log_level < Level::Info {
        return Err(LogLevelError::LogLevelTooHigh);
    }

    pretty_env_logger::init();
    Ok(())
}

// Define a custom error type
#[derive(Error, Debug)]
pub enum LogLevelError {
    #[error("Log level is higher than 'Info'. Please run with env flag RUST_LOG set to 'info' or lower")]
    LogLevelTooHigh,
    #[error("Failed to parse log level")]
    ParseError,
    #[error("Environment variable 'RUST_LOG' is not set")]
    EnvVarNotSet,
}