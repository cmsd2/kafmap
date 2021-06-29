use crate::error;

pub type Result<T> = anyhow::Result<T, error::Error>;
