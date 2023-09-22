use std::fmt;

pub struct ModelRepositoryError {}

impl fmt::Display for ModelRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform model repository operation")
    }
}

pub mod models;
