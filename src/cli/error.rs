use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct EarlyQuit;

impl Display for EarlyQuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Early quit")
    }
}

impl Error for EarlyQuit {}
